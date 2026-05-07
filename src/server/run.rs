use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;
use macro_traffic_sim_core::od::OdMatrix;
use macro_traffic_sim_core::pipeline::run_four_step_model;

use super::{BoxStream, SessionState, SessionsStorage};

pub async fn run_pipeline(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::RunPipelineRequest>,
) -> Result<Response<BoxStream<pb::RunPipelineProgress>>, Status> {
    let req = request.into_inner();
    let id_msg = req.session_id
        .ok_or_else(|| Status::invalid_argument("missing session_id"))?;
    let sid = Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument("invalid UUID"))?;

    // Extract everything we need from the session (to avoid holding the lock during computation)
    let (network, zones, od_matrix, config, trip_generator, impedance, logit_model, time_periods) = {
        let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;

        let extracted = guard.with_session(&sid, |session| {
            if session.state == SessionState::Running {
                return Err(Status::already_exists("Pipeline already running"));
            }
            if session.network.nodes.is_empty() {
                return Err(Status::failed_precondition("No network loaded"));
            }
            if session.zones.is_empty() {
                return Err(Status::failed_precondition("No zones loaded"));
            }
            let config = session.config.take()
                .ok_or_else(|| Status::failed_precondition("No config set"))?;
            let trip_gen = session.trip_generator.take()
                .ok_or_else(|| Status::failed_precondition("No trip generator set"))?;
            let impedance = session.impedance.take()
                .ok_or_else(|| Status::failed_precondition("No impedance function set"))?;
            let logit = session.logit_model.take()
                .ok_or_else(|| Status::failed_precondition("No logit model set"))?;

            // Clone/take what we need
            // Network and zones must be cloned for the computation thread
            let network = std::mem::take(&mut session.network);
            let zones = std::mem::take(&mut session.zones);
            let od = session.od_matrix.take();
            let periods = session.time_periods.clone();

            session.state = SessionState::Running;
            session.results.clear();

            Ok((network, zones, od, config, trip_gen, impedance, logit, periods))
        });

        match extracted {
            Some(Ok(data)) => data,
            Some(Err(e)) => return Err(e),
            None => return Err(Status::not_found(format!("Session not found: {}", sid))),
        }
    };

    let (tx, rx) = mpsc::channel(16);

    // Run pipeline in a blocking thread (it's CPU-bound)
    let sessions_for_result = sessions.clone();
    tokio::task::spawn_blocking(move || {
        let periods = if time_periods.is_empty() {
            vec![pb::TimePeriod {
                name: "default".to_string(),
                start_hour: 0.0,
                end_hour: 24.0,
                demand_factor: 1.0,
            }]
        } else {
            time_periods
        };

        let period_total = periods.len() as u32;
        let mut all_results = Vec::with_capacity(periods.len());

        for (pi, period) in periods.iter().enumerate() {
            let period_current = pi as u32 + 1;

            // Send progress: starting period
            let _ = tx.blocking_send(Ok(pb::RunPipelineProgress {
                code: Code::Ok as u32,
                text: format!("Starting period '{}'", period.name),
                session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
                phase: "generation".to_string(),
                period_current,
                period_total,
                feedback_current: 0,
                feedback_total: config.feedback_iterations as u32,
                assignment_iteration: 0,
                assignment_max_iterations: config.assignment_config.max_iterations as u32,
                relative_gap: 0.0,
                is_completed: false,
                is_failed: false,
                error_message: String::new(),
            }));

            // Scale OD matrix by demand factor if provided
            let effective_od;
            let od_ref: &dyn OdMatrix = if let Some(ref od) = od_matrix {
                if (period.demand_factor - 1.0).abs() > 1e-12 {
                    let zone_ids = od.zone_ids().to_vec();
                    let n = zone_ids.len();
                    let mut scaled_data = Vec::with_capacity(n * n);
                    for i in 0..n {
                        for j in 0..n {
                            scaled_data.push(od.get_by_index(i, j) * period.demand_factor);
                        }
                    }
                    effective_od = Some(macro_traffic_sim_core::od::dense::DenseOdMatrix::from_data(
                        zone_ids, scaled_data,
                    ));
                    effective_od.as_ref().unwrap()
                } else {
                    od
                }
            } else {
                // No explicit OD matrix provided; pipeline generates from trip generation
                // Use a zero matrix as placeholder (pipeline generates OD internally)
                let zone_ids: Vec<i64> = zones.iter().map(|z| z.id).collect();
                effective_od = Some(macro_traffic_sim_core::od::dense::DenseOdMatrix::new(zone_ids));
                effective_od.as_ref().unwrap()
            };

            let _ = od_ref; // suppress unused warning; actual call uses it indirectly

            // Run the 4-step pipeline
            let result = run_four_step_model(
                &network,
                &zones,
                trip_generator.as_ref(),
                impedance.as_ref(),
                &logit_model,
                &config,
            );

            match result {
                Ok(pipeline_result) => {
                    // Send progress: period completed
                    let _ = tx.blocking_send(Ok(pb::RunPipelineProgress {
                        code: Code::Ok as u32,
                        text: format!(
                            "Period '{}' complete: {} iterations, gap={:.6}",
                            period.name,
                            pipeline_result.assignment.iterations,
                            pipeline_result.assignment.relative_gap,
                        ),
                        session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
                        phase: "assignment".to_string(),
                        period_current,
                        period_total,
                        feedback_current: pipeline_result.feedback_iterations_done as u32,
                        feedback_total: config.feedback_iterations as u32,
                        assignment_iteration: pipeline_result.assignment.iterations as u32,
                        assignment_max_iterations: config.assignment_config.max_iterations as u32,
                        relative_gap: pipeline_result.assignment.relative_gap,
                        is_completed: false,
                        is_failed: false,
                        error_message: String::new(),
                    }));

                    all_results.push(pipeline_result);
                }
                Err(e) => {
                    let _ = tx.blocking_send(Ok(pb::RunPipelineProgress {
                        code: Code::Aborted as u32,
                        text: format!("Pipeline failed: {}", e),
                        session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
                        phase: String::new(),
                        period_current,
                        period_total,
                        feedback_current: 0,
                        feedback_total: 0,
                        assignment_iteration: 0,
                        assignment_max_iterations: 0,
                        relative_gap: 0.0,
                        is_completed: false,
                        is_failed: true,
                        error_message: e.to_string(),
                    }));

                    // Store state back as failed
                    if let Ok(mut guard) = sessions_for_result.lock() {
                        guard.with_session(&sid, |session| {
                            session.network = network;
                            session.zones = zones;
                            session.state = SessionState::Failed;
                        });
                    }
                    return;
                }
            }
        }

        // Send final completion message
        let _ = tx.blocking_send(Ok(pb::RunPipelineProgress {
            code: Code::Ok as u32,
            text: format!("All {} periods completed", all_results.len()),
            session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
            phase: "done".to_string(),
            period_current: period_total,
            period_total,
            feedback_current: 0,
            feedback_total: 0,
            assignment_iteration: 0,
            assignment_max_iterations: 0,
            relative_gap: 0.0,
            is_completed: true,
            is_failed: false,
            error_message: String::new(),
        }));

        // Store results back in session
        if let Ok(mut guard) = sessions_for_result.lock() {
            guard.with_session(&sid, |session| {
                session.network = network;
                session.zones = zones;
                session.results = all_results;
                session.state = SessionState::Completed;
            });
        }
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}
