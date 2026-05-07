# Go client for macro_traffic_sim gRPC Server

[![Go Reference](https://pkg.go.dev/badge/github.com/LdDl/macro_traffic_sim_grpc/clients/go.svg)](https://pkg.go.dev/github.com/LdDl/macro_traffic_sim_grpc/clients/go)

Go client library for the macroscopic traffic simulation gRPC server.

## Installation

```bash
go get github.com/LdDl/macro_traffic_sim_grpc/clients/go@latest
```

## Usage

```go
package main

import (
    "context"
    "fmt"

    macrotraffic "github.com/LdDl/macro_traffic_sim_grpc/clients/go"
    "google.golang.org/grpc"
    "google.golang.org/grpc/credentials/insecure"
)

func main() {
    conn, err := grpc.Dial("127.0.0.1:50052", grpc.WithTransportCredentials(insecure.NewCredentials()))
    if err != nil {
        panic(err)
    }
    defer conn.Close()

    client := macrotraffic.NewMacroServiceClient(conn)

    // Create a new session
    resp, err := client.NewSession(context.Background(), &macrotraffic.NewSessionRequest{})
    if err != nil {
        panic(err)
    }

    sessionID := resp.SessionId.Value
    fmt.Println("Session:", sessionID)
    // Push network, zones, configure, run pipeline, get results...
}
```

## Documentation

- **API reference**: https://pkg.go.dev/github.com/LdDl/macro_traffic_sim_grpc/clients/go

## Running the example

1. Start the gRPC server:
```bash
cargo run --features server --bin macro_traffic_sim
```

2. Run the example (from repository root):
```bash
go run -C clients/go/cmd/example .
```
