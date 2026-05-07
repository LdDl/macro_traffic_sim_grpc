# macro_traffic_sim_grpc

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

gRPC interface for macroscopic traffic simulation via 4-step demand model.

This crate wraps [`macro_traffic_sim_core`](https://github.com/LdDl/macro_traffic_sim_core) with a [tonic](https://docs.rs/tonic)-based gRPC API, enabling language-agnostic access from Go, Python, or any gRPC-compatible client.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Build](#build)
- [Run](#run)
- [Environment Variables](#environment-variables)
- [API Overview](#api-overview)
- [Usage Example](#usage-example)
- [Client Code Generation](#client-code-generation)
- [Related](#related)

## Features

- **Session-based** - Create isolated simulation sessions with UUID identifiers
- **Streaming** - Bidirectional streaming for large networks (nodes, links, zones, OD matrices)
- **4-step pipeline** - Full trip generation, distribution, mode choice, and traffic assignment
- **Progress reporting** - Server-side streaming of pipeline execution progress
- **Multi-period** - Run pipeline for multiple time periods with demand scaling
- **Results retrieval** - Link volumes, OD matrices, assignment convergence info

## Prerequisites

- Rust 1.85+ (edition 2024)
- Protocol Buffers compiler (`protoc`) for client generation

## Build

```sh
# Library only (for use as dependency)
cargo build

# Server binary
cargo build --features server --release
```

## Run

```sh
cargo run --features server --release
```

The server starts on `0.0.0.0:50052` by default.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `MACRO_SIM_ADDR` | `0.0.0.0:50052` | gRPC server listen address |
| `MACRO_SIM_SESSION_TTL` | `600` | Session time-to-live in seconds |

## API Overview

| RPC | Type | Description |
|-----|------|-------------|
| `NewSession` | Unary | Create a new simulation session |
| `InfoSession` | Unary | Get session state and statistics |
| `DeleteSession` | Unary | Delete a session and free resources |
| `PushNetwork` | Bidi stream | Load nodes and links (chunks of 10000) |
| `PushZones` | Bidi stream | Load transport analysis zones |
| `PushOdMatrix` | Bidi stream | Load OD demand matrix |
| `SetModelConfig` | Unary | Configure all 4-step model parameters |
| `RunPipeline` | Server stream | Execute pipeline with progress events |
| `GetLinkVolumes` | Server stream | Retrieve link volumes for a period |
| `GetSkimMatrix` | Server stream | Retrieve skim matrix (TODO) |
| `GetOdResult` | Server stream | Retrieve OD matrix result |
| `GetAssignmentInfo` | Unary | Get assignment convergence summary |

### Session Lifecycle

```
1. NewSession()                -> session_id
2. PushNetwork(nodes, links)   -> network loaded
3. PushZones(zones)            -> zones loaded
4. PushOdMatrix(matrix)        -> OD loaded
5. SetModelConfig(config)      -> configured
6. RunPipeline()               -> stream progress -> completed
7. GetLinkVolumes(period)      -> stream volumes
   GetOdResult(mode)           -> stream OD
   GetAssignmentInfo(period)   -> convergence info
8. DeleteSession()             -> cleanup
```

## Usage Example

See [`examples/rust_client/main.rs`](examples/rust_client/main.rs) for a complete working example.

```rust
use macro_traffic_sim::pb::macro_service_client::MacroServiceClient;
use macro_traffic_sim::pb::*;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50052")
        .connect()
        .await?;
    let mut client = MacroServiceClient::new(channel);

    // Create session
    let resp = client.new_session(NewSessionRequest {}).await?.into_inner();
    let sid = resp.session_id.unwrap().value;
    println!("Session: {}", sid);

    // Push network, zones, OD matrix, configure, run...
    Ok(())
}
```

## Client Code Generation

### Go

```sh
protoc --go_out=clients/go --go-grpc_out=clients/go protos/*.proto
```

### Python

```sh
python -m grpc_tools.protoc -Iprotos --python_out=clients/python --grpc_python_out=clients/python protos/*.proto
```

## Related

- [`macro_traffic_sim_core`](https://github.com/LdDl/macro_traffic_sim_core) - Computation engine (4-step demand model)
- [`micro_traffic_sim_grpc`](https://github.com/LdDl/micro_traffic_sim_grpc) - gRPC interface for microscopic simulation
- [`micro_traffic_sim_core`](https://github.com/LdDl/micro_traffic_sim_core) - Microscopic simulation engine (cellular automata)
