use std::collections::HashMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use futures_core::Stream;
use tokio::time::sleep;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use macro_traffic_sim::pb;
use macro_traffic_sim_core::config::ModelConfig;
use macro_traffic_sim_core::gmns::meso::network::Network;
use macro_traffic_sim_core::mode_choice::MultinomialLogit;
use macro_traffic_sim_core::od::dense::DenseOdMatrix;
use macro_traffic_sim_core::pipeline::PipelineResult;
use macro_traffic_sim_core::trip_distribution::impedance::ImpedanceFunction;
use macro_traffic_sim_core::trip_generation::TripGenerator;
use macro_traffic_sim_core::zone::Zone;

mod sessions;
mod network;
mod config;
mod run;
mod results;

pub(super) type BoxStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + 'static>>;

// Session state for macro simulation
pub struct MacroSession {
    pub id: Uuid,
    pub network: Network,
    pub zones: Vec<Zone>,
    pub od_matrix: Option<DenseOdMatrix>,
    pub config: Option<ModelConfig>,
    pub trip_generator: Option<Box<dyn TripGenerator + Send>>,
    pub impedance: Option<Box<dyn ImpedanceFunction + Send>>,
    pub logit_model: Option<MultinomialLogit>,
    pub time_periods: Vec<pb::TimePeriod>,
    pub results: Vec<PipelineResult>,
    pub state: SessionState,
    last_access: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Empty,
    NetworkLoaded,
    Configured,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SessionState::Empty => "empty",
            SessionState::NetworkLoaded => "network_loaded",
            SessionState::Configured => "configured",
            SessionState::Running => "running",
            SessionState::Completed => "completed",
            SessionState::Failed => "failed",
        };
        write!(f, "{}", s)
    }
}

impl MacroSession {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        MacroSession {
            id,
            network: Network::new(),
            zones: Vec::new(),
            od_matrix: None,
            config: None,
            trip_generator: None,
            impedance: None,
            logit_model: None,
            time_periods: Vec::new(),
            results: Vec::new(),
            state: SessionState::Empty,
            last_access: Instant::now(),
        }
    }

    pub fn touch(&mut self) {
        self.last_access = Instant::now();
    }

    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.last_access.elapsed() > ttl
    }
}

// Sessions storage
pub struct SessionsStorage {
    sessions: HashMap<Uuid, MacroSession>,
    ttl: Duration,
}

impl SessionsStorage {
    pub fn new(ttl: Duration) -> Self {
        SessionsStorage {
            sessions: HashMap::new(),
            ttl,
        }
    }

    pub fn register(&mut self, session: MacroSession) -> Uuid {
        let id = session.id;
        self.sessions.insert(id, session);
        id
    }

    pub fn with_session<F, R>(&mut self, id: &Uuid, f: F) -> Option<R>
    where
        F: FnOnce(&mut MacroSession) -> R,
    {
        if let Some(session) = self.sessions.get_mut(id) {
            session.touch();
            Some(f(session))
        } else {
            None
        }
    }

    pub fn remove(&mut self, id: &Uuid) -> bool {
        self.sessions.remove(id).is_some()
    }

    pub fn purge_expired(&mut self) {
        let ttl = self.ttl;
        self.sessions.retain(|_, s| !s.is_expired(ttl));
    }
}

struct MacroService {
    sessions: Arc<Mutex<SessionsStorage>>,
}

#[tonic::async_trait]
impl pb::macro_service_server::MacroService for MacroService {
    type PushNetworkStream = BoxStream<pb::NetworkChunkResponse>;
    type PushZonesStream = BoxStream<pb::ZoneChunkResponse>;
    type PushOdMatrixStream = BoxStream<pb::OdMatrixChunkResponse>;
    type RunPipelineStream = BoxStream<pb::RunPipelineProgress>;
    type GetLinkVolumesStream = BoxStream<pb::LinkVolumesChunk>;
    type GetSkimMatrixStream = BoxStream<pb::SkimMatrixChunk>;
    type GetOdResultStream = BoxStream<pb::OdResultChunk>;

    async fn new_session(
        &self,
        request: Request<pb::NewSessionRequest>,
    ) -> Result<Response<pb::NewSessionResponse>, Status> {
        sessions::new_session(self.sessions.clone(), request).await
    }

    async fn info_session(
        &self,
        request: Request<pb::SessionId>,
    ) -> Result<Response<pb::InfoSessionResponse>, Status> {
        sessions::info_session(self.sessions.clone(), request).await
    }

    async fn delete_session(
        &self,
        request: Request<pb::SessionId>,
    ) -> Result<Response<pb::DeleteSessionResponse>, Status> {
        sessions::delete_session(self.sessions.clone(), request).await
    }

    async fn push_network(
        &self,
        request: Request<tonic::Streaming<pb::NetworkChunk>>,
    ) -> Result<Response<Self::PushNetworkStream>, Status> {
        network::push_network(self.sessions.clone(), request).await
    }

    async fn push_zones(
        &self,
        request: Request<tonic::Streaming<pb::ZoneChunk>>,
    ) -> Result<Response<Self::PushZonesStream>, Status> {
        network::push_zones(self.sessions.clone(), request).await
    }

    async fn push_od_matrix(
        &self,
        request: Request<tonic::Streaming<pb::OdMatrixChunk>>,
    ) -> Result<Response<Self::PushOdMatrixStream>, Status> {
        network::push_od_matrix(self.sessions.clone(), request).await
    }

    async fn set_model_config(
        &self,
        request: Request<pb::ModelConfigRequest>,
    ) -> Result<Response<pb::ModelConfigResponse>, Status> {
        config::set_model_config(self.sessions.clone(), request).await
    }

    async fn run_pipeline(
        &self,
        request: Request<pb::RunPipelineRequest>,
    ) -> Result<Response<Self::RunPipelineStream>, Status> {
        run::run_pipeline(self.sessions.clone(), request).await
    }

    async fn get_link_volumes(
        &self,
        request: Request<pb::LinkVolumesRequest>,
    ) -> Result<Response<Self::GetLinkVolumesStream>, Status> {
        results::get_link_volumes(self.sessions.clone(), request).await
    }

    async fn get_skim_matrix(
        &self,
        request: Request<pb::SkimRequest>,
    ) -> Result<Response<Self::GetSkimMatrixStream>, Status> {
        results::get_skim_matrix(self.sessions.clone(), request).await
    }

    async fn get_od_result(
        &self,
        request: Request<pb::OdResultRequest>,
    ) -> Result<Response<Self::GetOdResultStream>, Status> {
        results::get_od_result(self.sessions.clone(), request).await
    }

    async fn get_assignment_info(
        &self,
        request: Request<pb::AssignmentInfoRequest>,
    ) -> Result<Response<pb::AssignmentInfoResponse>, Status> {
        results::get_assignment_info(self.sessions.clone(), request).await
    }
}

fn spawn_purge_task(sessions: Arc<Mutex<SessionsStorage>>) {
    tokio::spawn(async move {
        let interval = Duration::from_secs(30);
        loop {
            sleep(interval).await;
            if let Ok(mut guard) = sessions.lock() {
                guard.purge_expired();
            }
        }
    });
}

pub async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
    let default_addr = "0.0.0.0:50052";
    let addr: SocketAddr = std::env::var("MACRO_SIM_ADDR")
        .unwrap_or_else(|_| default_addr.to_string())
        .parse()?;

    let ttl_secs: u64 = std::env::var("MACRO_SIM_SESSION_TTL")
        .unwrap_or_else(|_| "600".to_string())
        .parse()
        .unwrap_or(600);

    let store = SessionsStorage::new(Duration::from_secs(ttl_secs));
    let sessions = Arc::new(Mutex::new(store));
    spawn_purge_task(sessions.clone());

    let svc = pb::macro_service_server::MacroServiceServer::new(MacroService {
        sessions: sessions.clone(),
    });

    println!("Starting macro_traffic_sim gRPC server on {}", addr);
    Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.ok();
            println!("\nShutting down gRPC server...");
        })
        .await?;
    Ok(())
}

pub fn run_blocking() {
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    if let Err(e) = rt.block_on(main_async()) {
        eprintln!("Server failed: {e}");
    }
}
