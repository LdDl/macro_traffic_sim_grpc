# macro_traffic_sim gRPC server

[![Crates.io](https://img.shields.io/crates/v/macro_traffic_sim.svg)](https://crates.io/crates/macro_traffic_sim)
[![Documentation](https://docs.rs/macro_traffic_sim/badge.svg)](https://docs.rs/macro_traffic_sim)
[![License](https://img.shields.io/crates/l/macro_traffic_sim.svg)](https://github.com/LdDl/macro_traffic_sim_grpc/blob/master/LICENSE)

This crate exposes the gRPC API for the macro traffic simulation via 4-step demand model. It can be used as a Rust library ([crates.io](https://crates.io/crates/macro_traffic_sim)), run as a server binary, and distributed via Docker. Go and Python client stubs can be generated from the same protos.

## Table of Contents
- [Prerequisites for building from source](#prerequisites-for-building-from-source)
- [Build and run (binary)](#build-and-run-binary)
- [Docker](#docker)
    - [Build and run locally](#build-and-run-locally)
    - [Pre-built image from registry](#pre-built-image-from-registry)
- [Pre-built binaries from GitHub releases page](#pre-built-binaries-from-github-releases-page)
- [Usage](#usage)
    - [Run server locally](#run-server-locally)
    - [Rust client to macro_traffic_sim gRPC server](#rust-client-to-macro_traffic_sim-grpc-server)
    - [Golang client to macro_traffic_sim gRPC server](#golang-client-to-macro_traffic_sim-grpc-server)
    - [Python client to macro_traffic_sim gRPC server](#python-client-to-macro_traffic_sim-grpc-server)
- [Client code generation](#client-code-generation)
    - [Golang](#golang)
    - [Python](#python)

## Prerequisites for building from source

- Rust 1.91.0 which is tested with 2024 edition in my case
- `protoc` available on PATH
- Optional: `docker` for container builds

## Build and run (binary)

- Debug build (library mode):
  - `make build`
- Run the gRPC server:
  - `make run-server`
- Release binary:
  - `make build-release`
  - Binary path: `target/release/macro_traffic_sim`

Notes:
- The server is behind a Cargo feature flag `server`. Commands above enable it when needed. Those are basically:
```sh
cargo build --release --features server
```
- Default listen address is `0.0.0.0:50052`. Override with `MACRO_SIM_ADDR` environment variable (e.g., `MACRO_SIM_ADDR=0.0.0.0:25250`).
- Session TTL is controlled by `MACRO_SIM_SESSION_TTL` environment variable (default: `600` seconds).

## Docker

There are two supported paths: build locally with Dockerfile, or pull from registry.

### Build and run locally

- Build
  - `make docker-build IMAGE=macro-traffic-sim/server TAG=latest`
- Run
  - `make docker-run IMAGE=macro-traffic-sim/server TAG=latest`
  - This maps host port 50052 -> container port 50052.

The Docker image is built with a multi-stage process (Rust builder + slim runtime). It compiles with the `server` feature enabled.

### Pre-built image from registry

The server image is available from both Docker Hub and GitHub Container Registry.

**Docker Hub:**
```sh
docker pull dimahkiin/macro-traffic-sim-server:latest
docker run --rm -it -p 50052:50052 -e MACRO_SIM_ADDR=0.0.0.0:50052 dimahkiin/macro-traffic-sim-server:latest
```

**GitHub Container Registry:**
```sh
docker pull ghcr.io/lddl/macro-traffic-sim-server:latest
docker run --rm -it -p 50052:50052 -e MACRO_SIM_ADDR=0.0.0.0:50052 ghcr.io/lddl/macro-traffic-sim-server:latest
```

Replace `latest` with a specific version tag (e.g., `0.1.0`) for reproducible deployments.

## Pre-built binaries from GitHub releases page

Download pre-built binaries from the [GitHub Releases page](https://github.com/LdDl/macro_traffic_sim_grpc/releases).

Available builds:
- **Linux (amd64):** `macro-traffic-sim-server-{version}-linux-amd64.tar.gz`
- **Windows (amd64):** `macro-traffic-sim-server-{version}-windows-amd64.zip`

**Linux example:**
```sh
# Download and extract
wget https://github.com/LdDl/macro_traffic_sim_grpc/releases/download/v0.1.0/macro-traffic-sim-server-0.1.0-linux-amd64.tar.gz
tar -xzf macro-traffic-sim-server-0.1.0-linux-amd64.tar.gz

# Run the server
./macro_traffic_sim
```

**Windows example:**
```powershell
# Extract the zip file, then run:
.\macro_traffic_sim.exe
```

The server listens on `0.0.0.0:50052` by default. Override with `MACRO_SIM_ADDR` environment variable (e.g., `MACRO_SIM_ADDR=0.0.0.0:25250`). Session TTL: `MACRO_SIM_SESSION_TTL` (default `600` seconds).

## Usage

### Run server locally

E.g. we can run the server in debug mode with:

```sh
cargo run --features server --bin macro_traffic_sim
```

To use a custom address and session TTL:
```sh
MACRO_SIM_ADDR=0.0.0.0:25250 MACRO_SIM_SESSION_TTL=1200 cargo run --features server --bin macro_traffic_sim
```

### Rust client to macro_traffic_sim gRPC server

Add the crate to your project: `cargo add macro_traffic_sim`

- [API Documentation (docs.rs)](https://docs.rs/macro_traffic_sim)
- [Example details](./examples/rust_client/README.md)

```sh
export MACRO_SIM_ADDR=127.0.0.1:50052
cargo run --example rust_client
```

### Golang client to macro_traffic_sim gRPC server

Here more details: [clients/go/README.md](./clients/go/README.md)

```sh
export MACRO_SIM_ADDR=127.0.0.1:50052
# from repository root
cd ./clients/go
go run ./cmd/example/main.go
```

### Python client to macro_traffic_sim gRPC server

Here more details: [clients/python/README.md](./clients/python/README.md)

```sh
export MACRO_SIM_ADDR=127.0.0.1:50052
# from repository root
cd ./clients/python
source .venv/bin/activate
python examples/main.py
```

## Client code generation

This section describes how I've used to generate client code for different languages from the proto files.

### Golang
Client code generation for Golang is done via [scripts/gen_go.sh](./scripts/gen_go.sh). It requires `protoc` and `protoc-gen-go` to be installed and available on PATH.
```sh
chmod +x ./scripts/gen_go.sh
./scripts/gen_go.sh clients/go
cd ./clients/go
go mod init github.com/LdDl/macro_traffic_sim_grpc/clients/go
go mod tidy
cd -
```

### Python

Client code generation for Python is done via [scripts/gen_python.sh](./scripts/gen_python.sh). The script automatically creates a virtual environment and installs all dependencies.

```sh
chmod +x ./scripts/gen_python.sh
./scripts/gen_python.sh
```

The script:
1. Creates `.venv` in `clients/python/` (if not exists)
2. Installs dependencies from `requirements.txt`
3. Generates `*_pb2.py`, `*_pb2.pyi` (type stubs), and `*_pb2_grpc.py`
4. Installs the `macro-traffic-sim` package in editable mode
