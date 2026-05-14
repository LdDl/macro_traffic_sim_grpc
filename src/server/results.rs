use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;
use macro_traffic_sim_core::gmns::types::AgentType;
use macro_traffic_sim_core::od::OdMatrix;

use super::{BoxStream, SessionState, SessionsStorage};

fn parse_session_id_from_uuid(id: &Option<pb::UuiDv4>) -> Result<Uuid, Status> {
    let id_msg = id.as_ref()
        .ok_or_else(|| Status::invalid_argument("missing session_id"))?;
    Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument(format!("Invalid UUID: '{}'", id_msg.value)))
}

fn mode_str_to_agent_type(mode: &str) -> Option<AgentType> {
    match mode {
        "auto" => Some(AgentType::Auto),
        "bike" => Some(AgentType::Bike),
        "walk" => Some(AgentType::Walk),
        _ => None,
    }
}

pub async fn get_link_volumes(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::LinkVolumesRequest>,
) -> Result<Response<BoxStream<pb::LinkVolumesChunk>>, Status> {
    let req = request.into_inner();
    let sid = parse_session_id_from_uuid(&req.session_id)?;
    let period_index = req.period_index as usize;

    let (tx, rx) = mpsc::channel(16);

    let volumes_data = {
        let mut g = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
        g.with_session(&sid, |session| {
            if session.state != SessionState::Completed {
                return Err(Status::failed_precondition("Pipeline has not completed"));
            }
            if period_index >= session.results.len() {
                return Err(Status::invalid_argument(format!(
                    "Period index {} out of range (max {})",
                    period_index,
                    session.results.len().saturating_sub(1)
                )));
            }

            let result = &session.results[period_index];
            let volumes: Vec<pb::LinkVolume> = result.assignment.link_volumes.iter().map(|(&link_id, &volume)| {
                let cost = result.assignment.link_costs.get(&link_id).copied().unwrap_or(0.0);
                let v_over_c = session.network.get_link(link_id)
                    .map(|link| {
                        let cap = link.get_total_capacity();
                        if cap > 0.0 { volume / cap } else { 0.0 }
                    })
                    .unwrap_or(0.0);
                pb::LinkVolume {
                    link_id,
                    volume,
                    travel_time: cost,
                    v_over_c,
                }
            }).collect();

            Ok(volumes)
        })
    };

    match volumes_data {
        None => return Err(Status::not_found(format!("Session not found: {}", sid))),
        Some(Err(e)) => return Err(e),
        Some(Ok(volumes)) => {
            tokio::spawn(async move {
                // Stream in chunks of 5000
                for chunk in volumes.chunks(5000) {
                    let resp = pb::LinkVolumesChunk {
                        code: Code::Ok as u32,
                        text: "OK".to_string(),
                        period_index: period_index as u32,
                        volumes: chunk.to_vec(),
                    };
                    if tx.send(Ok(resp)).await.is_err() { return; }
                }
            });
        }
    }

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}

pub async fn get_skim_matrix(
    _sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::SkimRequest>,
) -> Result<Response<BoxStream<pb::SkimMatrixChunk>>, Status> {
    let _req = request.into_inner();

    let (tx, rx) = mpsc::channel(16);

    let _ = tx.send(Err(Status::unimplemented(
        "GetSkimMatrix: not yet implemented. Use GetLinkVolumes for assignment results."
    ))).await;

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}

pub async fn get_od_result(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::OdResultRequest>,
) -> Result<Response<BoxStream<pb::OdResultChunk>>, Status> {
    let req = request.into_inner();
    let sid = parse_session_id_from_uuid(&req.session_id)?;
    let mode = req.mode.clone();

    let (tx, rx) = mpsc::channel(16);

    let od_data = {
        let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
        guard.with_session(&sid, |session| {
            if session.state != SessionState::Completed || session.results.is_empty() {
                return Err(Status::failed_precondition("Pipeline has not completed"));
            }

            let result = &session.results[0]; // Use first period

            if mode.is_empty() {
                // Return total OD
                let zone_ids = result.total_od.zone_ids().to_vec();
                let data = result.total_od.data().to_vec();
                Ok((zone_ids, data))
            } else {
                let agent_type = mode_str_to_agent_type(&mode)
                    .ok_or_else(|| Status::invalid_argument(format!("Unknown mode: {}", mode)))?;
                let mode_od = result.mode_od.get(&agent_type)
                    .ok_or_else(|| Status::not_found(format!("No OD for mode: {}", mode)))?;
                let zone_ids = mode_od.zone_ids().to_vec();
                let data = mode_od.data().to_vec();
                Ok((zone_ids, data))
            }
        })
    };

    match od_data {
        None => return Err(Status::not_found(format!("Session not found: {}", sid))),
        Some(Err(e)) => return Err(e),
        Some(Ok((zone_ids, data))) => {
            tokio::spawn(async move {
                // Send zone_ids in first chunk, data in subsequent chunks
                let chunk_size = 10000;
                let mut first = true;
                for chunk in data.chunks(chunk_size) {
                    let resp = pb::OdResultChunk {
                        code: Code::Ok as u32,
                        text: "OK".to_string(),
                        zone_ids: if first { zone_ids.clone() } else { vec![] },
                        data: chunk.to_vec(),
                    };
                    first = false;
                    if tx.send(Ok(resp)).await.is_err() { return; }
                }
            });
        }
    }

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}

pub async fn get_assignment_info(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::AssignmentInfoRequest>,
) -> Result<Response<pb::AssignmentInfoResponse>, Status> {
    let req = request.into_inner();
    let sid = parse_session_id_from_uuid(&req.session_id)?;
    let period_index = req.period_index as usize;

    let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
    let result = guard.with_session(&sid, |session| {
        if session.state != SessionState::Completed {
            return Err(Status::failed_precondition("Pipeline has not completed"));
        }
        if period_index >= session.results.len() {
            return Err(Status::invalid_argument(format!(
                "Period index {} out of range", period_index
            )));
        }

        let r = &session.results[period_index];
        Ok(pb::AssignmentInfoResponse {
            code: Code::Ok as u32,
            text: "OK".to_string(),
            info: Some(pb::AssignmentInfo {
                iterations: r.assignment.iterations as u32,
                relative_gap: r.assignment.relative_gap,
                converged: r.assignment.converged,
            }),
        })
    });
    drop(guard);

    match result {
        None => Err(Status::not_found(format!("Session not found: {}", sid))),
        Some(Err(e)) => Err(e),
        Some(Ok(resp)) => Ok(Response::new(resp)),
    }
}
