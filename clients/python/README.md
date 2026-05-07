# Python client for macro_traffic_sim gRPC Server

[![Python 3.10+](https://img.shields.io/badge/python-3.10%2B-blue.svg)](https://www.python.org/downloads/)
[![Typing: Typed](https://img.shields.io/badge/typing-typed-green.svg)](https://peps.python.org/pep-0561/)

Python client library for the macroscopic traffic simulation gRPC server with full type hints support.

## Installation

```bash
pip install macro-traffic-sim
```

## Usage

```python
import grpc
from macro_traffic_sim import (
    MacroServiceStub,
    NewSessionRequest,
    Node,
    Link,
    Zone,
    NetworkChunk,
    ZoneChunk,
    ModelConfigRequest,
    BprConfig,
    RunPipelineRequest,
    LinkVolumesRequest,
    UUIDv4,
)

# Connect to server
channel = grpc.insecure_channel("127.0.0.1:50052")
client = MacroServiceStub(channel)

# Create a new session
response = client.NewSession(NewSessionRequest())
session_id = response.session_id.value
print(f"Session: {session_id}")

# Push network, zones, configure, run pipeline, get results...
```

## Documentation

- **Full example**: See [examples/](https://github.com/LdDl/macro_traffic_sim_grpc/tree/master/clients/python/examples) for a complete simulation workflow

## Running the example

1. Start the gRPC server:
```bash
cargo run --features server --bin macro_traffic_sim
```

2. Run the example (from repository root):
```bash
source clients/python/.venv/bin/activate
python clients/python/examples/main.py
```
