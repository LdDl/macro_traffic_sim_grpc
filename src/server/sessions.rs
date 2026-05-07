use std::sync::{Arc, Mutex};

use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;

use super::{MacroSession, SessionsStorage};

pub async fn new_session(
    sessions: Arc<Mutex<SessionsStorage>>,
    _request: Request<pb::NewSessionRequest>,
) -> Result<Response<pb::NewSessionResponse>, Status> {
    let session = MacroSession::new();
    let sid = session.id;

    let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
    guard.register(session);
    drop(guard);

    let resp = pb::NewSessionResponse {
        code: Code::Ok as u32,
        text: "OK".to_string(),
        session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
    };
    Ok(Response::new(resp))
}

pub async fn info_session(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::SessionId>,
) -> Result<Response<pb::InfoSessionResponse>, Status> {
    let id_msg = request.into_inner().value
        .ok_or_else(|| Status::invalid_argument("missing session_id"))?;
    let sid = Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument("invalid UUID"))?;

    let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
    let result = guard.with_session(&sid, |session| {
        pb::InfoSessionResponse {
            code: Code::Ok as u32,
            text: "OK".to_string(),
            session_id: Some(pb::UuiDv4 { value: sid.to_string() }),
            state: session.state.to_string(),
            has_network: !session.network.nodes.is_empty(),
            has_zones: !session.zones.is_empty(),
            has_od_matrix: session.od_matrix.is_some(),
            has_config: session.config.is_some(),
            has_results: !session.results.is_empty(),
            num_nodes: session.network.node_count() as u32,
            num_links: session.network.link_count() as u32,
            num_zones: session.zones.len() as u32,
        }
    });
    drop(guard);

    match result {
        Some(resp) => Ok(Response::new(resp)),
        None => {
            let resp = pb::InfoSessionResponse {
                code: Code::NotFound as u32,
                text: format!("Session not found: {}", sid),
                session_id: None,
                state: String::new(),
                has_network: false,
                has_zones: false,
                has_od_matrix: false,
                has_config: false,
                has_results: false,
                num_nodes: 0,
                num_links: 0,
                num_zones: 0,
            };
            Ok(Response::new(resp))
        }
    }
}

pub async fn delete_session(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<pb::SessionId>,
) -> Result<Response<pb::DeleteSessionResponse>, Status> {
    let id_msg = request.into_inner().value
        .ok_or_else(|| Status::invalid_argument("missing session_id"))?;
    let sid = Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument("invalid UUID"))?;

    let mut guard = sessions.lock().map_err(|_| Status::internal("storage poisoned"))?;
    let removed = guard.remove(&sid);
    drop(guard);

    if removed {
        Ok(Response::new(pb::DeleteSessionResponse {
            code: Code::Ok as u32,
            text: "OK".to_string(),
        }))
    } else {
        Err(Status::not_found(format!("Session not found: {}", sid)))
    }
}
