//! # macro_traffic_sim
//!
//! gRPC interface for macroscopic traffic simulation via 4-step demand model.
//!
//! This crate provides a [tonic]-based gRPC client and server for interacting with
//! the [`macro_traffic_sim_core`] computation engine. It allows you to:
//!
//! - Create simulation sessions
//! - Define transport networks (meso nodes, links, zones)
//! - Load origin-destination demand matrices
//! - Configure the 4-step model (trip generation, distribution, mode choice, assignment)
//! - Run the pipeline and observe progress
//! - Retrieve results (link volumes, OD matrices, convergence info)
//!
//! ## Architecture
//!
//! The computation core ([`macro_traffic_sim_core`]) implements the classical
//! 4-step traffic demand model (Generation, Distribution, Mode Choice, Assignment).
//! This crate wraps it with a gRPC API defined in Protocol Buffers, enabling
//! language-agnostic access from Go, Python, or any gRPC-compatible client.
//!
//! ## Quick Start (Client)
//!
//! ```rust,no_run
//! use macro_traffic_sim::pb::macro_service_client::MacroServiceClient;
//! use macro_traffic_sim::pb::*;
//! use tonic::transport::Channel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let channel = Channel::from_static("http://127.0.0.1:50052")
//!         .connect()
//!         .await?;
//!     let mut client = MacroServiceClient::new(channel);
//!
//!     // Create a new session
//!     let response = client.new_session(NewSessionRequest {}).await?;
//!     let session_id = response.into_inner().session_id.unwrap().value;
//!     println!("Session created: {}", session_id);
//!
//!     // Now push network, zones, OD matrix, configure, and run...
//!     Ok(())
//! }
//! ```
//!
//! For a complete working example, see
//! [`examples/rust_client`](https://github.com/LdDl/macro_traffic_sim_grpc/tree/master/examples/rust_client).
//!
//! ## Running the Server
//!
//! The server binary is included when built with the `server` feature:
//!
//! ```sh
//! cargo run --features server --bin macro_traffic_sim
//! ```
//!
//! ## Protocol Buffers
//!
//! All types are generated from `.proto` files and exposed under the [`pb`] module:
//!
//! - [`pb::macro_service_client::MacroServiceClient`] - gRPC client stub
//! - [`pb::macro_service_server::MacroServiceServer`] - gRPC server trait (with `server` feature)
//! - **Session**: [`pb::NewSessionRequest`], [`pb::NewSessionResponse`], [`pb::InfoSessionResponse`]
//! - **Network**: [`pb::Node`], [`pb::Link`], [`pb::Zone`], [`pb::NetworkChunk`]
//! - **OD Matrix**: [`pb::OdMatrixChunk`]
//! - **Config**: [`pb::ModelConfigRequest`], [`pb::BprConfig`], [`pb::ModeUtility`], [`pb::TimePeriod`]
//! - **Pipeline**: [`pb::RunPipelineRequest`], [`pb::RunPipelineProgress`]
//! - **Results**: [`pb::LinkVolumesChunk`], [`pb::LinkVolume`], [`pb::OdResultChunk`], [`pb::AssignmentInfo`]
//!
//! ## Related Crates
//!
//! - [`macro_traffic_sim_core`] - The computation engine (4-step demand model)
//! - [`micro_traffic_sim`](https://docs.rs/micro_traffic_sim) - gRPC interface for microscopic simulation
//! - [`micro_traffic_sim_core`](https://docs.rs/micro_traffic_sim_core) - Microscopic engine (cellular automata)
//!
//! ## Clients in Other Languages
//!
//! - **Go**: [clients/go](https://github.com/LdDl/macro_traffic_sim_grpc/tree/master/clients/go)
//! - **Python**: [clients/python](https://github.com/LdDl/macro_traffic_sim_grpc/tree/master/clients/python)
//!
//! [`macro_traffic_sim_core`]: https://docs.rs/macro_traffic_sim_core/latest/macro_traffic_sim_core/
//! [tonic]: https://docs.rs/tonic/latest/tonic/

/// Generated Protocol Buffer types and gRPC service definitions.
///
/// This module contains all types generated from the `.proto` files:
///
/// - **Session management**: [`NewSessionRequest`], [`NewSessionResponse`],
///   [`SessionId`], [`InfoSessionResponse`], [`DeleteSessionResponse`]
/// - **Network**: [`Node`], [`Link`], [`Zone`], [`NetworkChunk`],
///   [`ZoneChunk`], [`OdMatrixChunk`]
/// - **Configuration**: [`ModelConfigRequest`], [`BprConfig`],
///   [`AssignmentConvergence`], [`FurnessConfig`], [`ImpedanceFunction`],
///   [`TripGenerationConfig`], [`RegressionCoeffs`], [`ModeUtility`], [`TimePeriod`]
/// - **Pipeline**: [`RunPipelineRequest`], [`RunPipelineProgress`]
/// - **Results**: [`LinkVolumesRequest`], [`LinkVolumesChunk`], [`LinkVolume`],
///   [`SkimRequest`], [`SkimMatrixChunk`], [`OdResultRequest`], [`OdResultChunk`],
///   [`AssignmentInfoRequest`], [`AssignmentInfoResponse`], [`AssignmentInfo`]
/// - **gRPC Client**: [`macro_service_client::MacroServiceClient`]
/// - **gRPC Server**: [`macro_service_server::MacroServiceServer`] (with `server` feature)
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/macro_traffic_sim.rs"));
}

pub use pb::*;
