use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;
use macro_traffic_sim_core::gmns::meso::link::Link;
use macro_traffic_sim_core::gmns::meso::node::Node;
use macro_traffic_sim_core::gmns::types::LinkType;
use macro_traffic_sim_core::od::dense::DenseOdMatrix;
use macro_traffic_sim_core::zone::Zone;

use super::{BoxStream, SessionState, SessionsStorage};

fn proto_link_type_to_core(lt: i32) -> LinkType {
    match lt {
        1 => LinkType::Motorway,
        2 => LinkType::Trunk,
        3 => LinkType::Primary,
        4 => LinkType::Secondary,
        5 => LinkType::Tertiary,
        6 => LinkType::Residential,
        7 => LinkType::LivingStreet,
        8 => LinkType::Service,
        9 => LinkType::Cycleway,
        10 => LinkType::Footway,
        11 => LinkType::Track,
        12 => LinkType::Unclassified,
        13 => LinkType::Connector,
        14 => LinkType::Railway,
        15 => LinkType::Aeroway,
        _ => LinkType::Undefined,
    }
}

fn parse_session_id(id: &Option<pb::UuiDv4>) -> Result<Uuid, Status> {
    let id_msg = id.as_ref()
        .ok_or_else(|| Status::invalid_argument("No session ID provided"))?;
    Uuid::parse_str(&id_msg.value)
        .map_err(|_| Status::invalid_argument(format!("Invalid UUID: '{}'", id_msg.value)))
}

pub async fn push_network(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<tonic::Streaming<pb::NetworkChunk>>,
) -> Result<Response<BoxStream<pb::NetworkChunkResponse>>, Status> {
    let mut stream = request.into_inner();
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        while let Ok(Some(req)) = stream.message().await {
            let sid = match parse_session_id(&req.session_id) {
                Ok(s) => s,
                Err(e) => { let _ = tx.send(Err(e)).await; return; }
            };

            let total_entities = req.nodes.len() + req.links.len();
            if total_entities > 10000 {
                let _ = tx.send(Err(Status::invalid_argument(format!(
                    "Max 10000 entities per chunk, got {}", total_entities
                )))).await;
                return;
            }

            if total_entities == 0 {
                let _ = tx.send(Err(Status::invalid_argument("No data provided"))).await;
                return;
            }

            // Use block scope to ensure MutexGuard is dropped before any .await
            let add_result = {
                let mut guard = sessions.lock().unwrap();
                guard.with_session(&sid, |session| {
                    let mut errors = Vec::new();

                    for n in &req.nodes {
                        let mut builder = Node::new(n.id)
                            .with_coordinates(n.latitude, n.longitude);
                        if n.zone_id >= 0 {
                            builder = builder.with_zone_id(n.zone_id);
                        }
                        if n.macro_node_id >= 0 {
                            builder = builder.with_macro_node_id(n.macro_node_id);
                        }
                        if n.macro_link_id >= 0 {
                            builder = builder.with_macro_link_id(n.macro_link_id);
                        }
                        if let Err(e) = session.network.add_node(builder.build()) {
                            errors.push(format!("node {}: {}", n.id, e));
                        }
                    }

                    for l in &req.links {
                        let mut builder = Link::new(l.id, l.source_node_id, l.target_node_id)
                            .with_length_meters(l.length_meters)
                            .with_free_speed(l.free_speed)
                            .with_capacity(l.capacity)
                            .with_lanes_num(l.lanes)
                            .with_link_type(proto_link_type_to_core(l.link_type))
                            .with_is_connection(l.is_connection);
                        if l.macro_link_id >= 0 {
                            builder = builder.with_macro_link_id(l.macro_link_id);
                        }
                        if l.movement_id >= 0 {
                            builder = builder.with_movement_id(l.movement_id);
                        }
                        if let Err(e) = session.network.add_link(builder.build()) {
                            errors.push(format!("link {}: {}", l.id, e));
                        }
                    }

                    if !session.network.nodes.is_empty() {
                        session.state = SessionState::NetworkLoaded;
                    }

                    (session.network.node_count(), session.network.link_count(), errors)
                })
            };

            match add_result {
                None => {
                    let _ = tx.send(Err(Status::not_found(format!(
                        "Session not found: {}", sid
                    )))).await;
                    return;
                }
                Some((nodes, links, errors)) => {
                    let msg = if errors.is_empty() {
                        "OK".to_string()
                    } else {
                        format!("OK with warnings: {}", errors.join("; "))
                    };
                    let resp = pb::NetworkChunkResponse {
                        code: Code::Ok as u32,
                        text: msg,
                        nodes_received: nodes as u32,
                        links_received: links as u32,
                    };
                    if tx.send(Ok(resp)).await.is_err() { break; }
                }
            }
        }
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}

pub async fn push_zones(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<tonic::Streaming<pb::ZoneChunk>>,
) -> Result<Response<BoxStream<pb::ZoneChunkResponse>>, Status> {
    let mut stream = request.into_inner();
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        while let Ok(Some(req)) = stream.message().await {
            let sid = match parse_session_id(&req.session_id) {
                Ok(s) => s,
                Err(e) => { let _ = tx.send(Err(e)).await; return; }
            };

            if req.zones.is_empty() {
                let _ = tx.send(Err(Status::invalid_argument("No zone data"))).await;
                return;
            }

            if req.zones.len() > 10000 {
                let _ = tx.send(Err(Status::invalid_argument(format!(
                    "Max 10000 zones per chunk, got {}", req.zones.len()
                )))).await;
                return;
            }

            let add_result = {
                let mut guard = sessions.lock().unwrap();
                guard.with_session(&sid, |session| {
                    for z in &req.zones {
                        let zone = Zone::new(z.id)
                            .with_name(&z.name)
                            .with_population(z.population)
                            .with_employment(z.employment)
                            .with_households(z.households)
                            .with_avg_income(z.avg_income)
                            .with_area_sq_km(z.area_sq_km)
                            .build();
                        session.zones.push(zone);
                    }
                    session.zones.len()
                })
            };

            match add_result {
                None => {
                    let _ = tx.send(Err(Status::not_found(format!(
                        "Session not found: {}", sid
                    )))).await;
                    return;
                }
                Some(total) => {
                    let resp = pb::ZoneChunkResponse {
                        code: Code::Ok as u32,
                        text: "OK".to_string(),
                        zones_received: total as u32,
                    };
                    if tx.send(Ok(resp)).await.is_err() { break; }
                }
            }
        }
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}

pub async fn push_od_matrix(
    sessions: Arc<Mutex<SessionsStorage>>,
    request: Request<tonic::Streaming<pb::OdMatrixChunk>>,
) -> Result<Response<BoxStream<pb::OdMatrixChunkResponse>>, Status> {
    let mut stream = request.into_inner();
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut zone_ids: Option<Vec<i64>> = None;
        let mut data_buf: Vec<f64> = Vec::new();

        while let Ok(Some(req)) = stream.message().await {
            let sid = match parse_session_id(&req.session_id) {
                Ok(s) => s,
                Err(e) => { let _ = tx.send(Err(e)).await; return; }
            };

            if !req.zone_ids.is_empty() {
                zone_ids = Some(req.zone_ids.clone());
            }

            data_buf.extend_from_slice(&req.data);

            let zids = match &zone_ids {
                Some(z) => z,
                None => {
                    let _ = tx.send(Err(Status::invalid_argument(
                        "First OD chunk must include zone_ids"
                    ))).await;
                    return;
                }
            };

            let expected = zids.len() * zids.len();

            let resp = pb::OdMatrixChunkResponse {
                code: Code::Ok as u32,
                text: format!("Received {}/{} cells", data_buf.len(), expected),
                cells_received: data_buf.len() as u64,
            };
            if tx.send(Ok(resp)).await.is_err() { return; }

            if data_buf.len() >= expected {
                let matrix_data = data_buf[..expected].to_vec();
                let zids_clone = zids.clone();

                let set_result = {
                    let mut guard = sessions.lock().unwrap();
                    guard.with_session(&sid, |session| {
                        let od = DenseOdMatrix::from_data(zids_clone, matrix_data);
                        session.od_matrix = Some(od);
                    })
                };

                if set_result.is_none() {
                    let _ = tx.send(Err(Status::not_found(format!(
                        "Session not found: {}", sid
                    )))).await;
                    return;
                }
            }
        }
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
}
