use std::sync::{Arc, Mutex};

use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;
use macro_traffic_sim_core::config::{AssignmentMethodType, ModelConfig};
use macro_traffic_sim_core::gmns::types::AgentType;
use macro_traffic_sim_core::mode_choice::logit::MultinomialLogit;
use macro_traffic_sim_core::mode_choice::utility::ModeUtility;
use macro_traffic_sim_core::trip_distribution::impedance::{
    CombinedImpedance, ExponentialImpedance, PowerImpedance,
};
use macro_traffic_sim_core::trip_generation::regression::{
    RegressionCoefficients, RegressionGenerator,
};
use macro_traffic_sim_core::trip_generation::TripGenerator;
use macro_traffic_sim_core::verbose::VerboseLevel;

use super::{SessionState, SessionsStorage};

fn proto_assignment_method(m: i32) -> AssignmentMethodType {
    match m {
        1 => AssignmentMethodType::Msa,
        2 => AssignmentMethodType::GradientProjection,
        _ => AssignmentMethodType::FrankWolfe,
    }
}

fn proto_mode_to_agent_type(mode: &str) -> AgentType {
    match mode {
        "auto" => AgentType::Auto,
        "bike" => AgentType::Bike,
        "walk" => AgentType::Walk,
        _ => AgentType::Undefined,
    }
}

pub async fn set_model_config(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::ModelConfigRequest>,
) -> Result<Response<pb::ModelConfigResponse>, Status> {
    let req = request.into_inner();

    let id_msg = req.session_id
        .ok_or_else(|| Status::invalid_argument("missing session_id"))?;
    let sid = Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument("invalid UUID"))?;

    // Build ModelConfig
    let mut config_builder = ModelConfig::new()
        .with_assignment_method(proto_assignment_method(req.assignment_method))
        .with_feedback_iterations(req.feedback_iterations.max(1) as usize)
        .with_gp_step_scale(if req.gp_step_scale > 0.0 { req.gp_step_scale } else { 0.1 })
        .with_verbose_level(VerboseLevel::None);

    if let Some(bpr) = &req.bpr {
        let alpha = if bpr.alpha > 0.0 { bpr.alpha } else { 0.15 };
        let beta = if bpr.beta > 0.0 { bpr.beta } else { 4.0 };
        config_builder = config_builder.with_bpr(alpha, beta);
    }

    if let Some(asgn) = &req.assignment {
        if asgn.max_iterations > 0 {
            config_builder = config_builder.with_max_iterations(asgn.max_iterations as usize);
        }
        if asgn.convergence_gap > 0.0 {
            config_builder = config_builder.with_convergence_gap(asgn.convergence_gap);
        }
    }

    if let Some(furness) = &req.furness {
        if furness.max_iterations > 0 {
            config_builder = config_builder.with_furness_max_iterations(furness.max_iterations as usize);
        }
        if furness.tolerance > 0.0 {
            config_builder = config_builder.with_furness_tolerance(furness.tolerance);
        }
    }

    let model_config = config_builder.build();

    // Build trip generator
    let trip_gen: Box<dyn TripGenerator + Send> = if let Some(tg) = &req.trip_generation {
        match tg.method {
            // TRIP_GEN_REGRESSION = 0
            0 => {
                let prod = tg.production_coeffs.as_ref().map(|c| RegressionCoefficients {
                    intercept: c.intercept,
                    pop_coeff: c.pop_coeff,
                    emp_coeff: c.emp_coeff,
                    hh_coeff: c.hh_coeff,
                    income_coeff: c.income_coeff,
                }).unwrap_or_default();

                let attr = tg.attraction_coeffs.as_ref().map(|c| RegressionCoefficients {
                    intercept: c.intercept,
                    pop_coeff: c.pop_coeff,
                    emp_coeff: c.emp_coeff,
                    hh_coeff: c.hh_coeff,
                    income_coeff: c.income_coeff,
                }).unwrap_or(RegressionCoefficients {
                    intercept: 0.0,
                    pop_coeff: 0.1,
                    emp_coeff: 0.8,
                    hh_coeff: 0.0,
                    income_coeff: 0.0,
                });

                Box::new(RegressionGenerator::with_coefficients(prod, attr))
            }
            // TRIP_GEN_CROSS_CLASSIFICATION = 1
            // TODO: implement cross-classification when core supports it
            _ => Box::new(RegressionGenerator::new()),
        }
    } else {
        Box::new(RegressionGenerator::new())
    };

    // Build impedance function
    let impedance: Box<dyn macro_traffic_sim_core::trip_distribution::impedance::ImpedanceFunction + Send> =
        if let Some(imp) = &req.impedance {
            match imp.r#type {
                // IMPEDANCE_POWER = 1
                1 => {
                    let alpha = if imp.alpha > 0.0 { imp.alpha } else { 2.0 };
                    Box::new(PowerImpedance::new(alpha))
                }
                // IMPEDANCE_COMBINED = 2
                2 => {
                    let alpha = if imp.alpha > 0.0 { imp.alpha } else { 1.5 };
                    let beta = if imp.beta > 0.0 { imp.beta } else { 0.05 };
                    Box::new(CombinedImpedance::new(alpha, beta))
                }
                // IMPEDANCE_EXPONENTIAL = 0 (default)
                _ => {
                    let beta = if imp.beta > 0.0 { imp.beta } else { 0.1 };
                    Box::new(ExponentialImpedance::new(beta))
                }
            }
        } else {
            Box::new(ExponentialImpedance::new(0.1))
        };

    // Build logit model
    let logit_model = if !req.mode_utilities.is_empty() {
        let utils: Vec<ModeUtility> = req.mode_utilities.iter().map(|mu| {
            let agent_type = proto_mode_to_agent_type(&mu.mode);
            ModeUtility::new(agent_type)
                .with_asc(mu.asc)
                .with_coeff_time(mu.coeff_time)
                .with_coeff_distance(mu.coeff_distance)
                .with_coeff_cost(mu.coeff_cost)
                .build()
        }).collect();
        MultinomialLogit::new(utils)
    } else {
        MultinomialLogit::default_auto_bike_walk()
    };

    // Store everything in session
    let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
    let result = guard.with_session(&sid, |session| {
        session.config = Some(model_config);
        session.trip_generator = Some(trip_gen);
        session.impedance = Some(impedance);
        session.logit_model = Some(logit_model);
        session.time_periods = req.time_periods.clone();
        session.state = SessionState::Configured;
    });
    drop(guard);

    match result {
        Some(()) => Ok(Response::new(pb::ModelConfigResponse {
            code: Code::Ok as u32,
            text: "OK".to_string(),
        })),
        None => Err(Status::not_found(format!("Session not found: {}", sid))),
    }
}
