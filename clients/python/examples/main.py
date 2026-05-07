#!/usr/bin/env python3
"""
Complete macroscopic traffic simulation example using the Python gRPC client.
Demonstrates full session lifecycle: create -> network -> zones -> config -> run -> results -> delete.
"""

from __future__ import annotations

import os

import grpc

from macro_traffic_sim import (
    AssignmentConvergence,
    AssignmentInfoRequest,
    BprConfig,
    DeleteSessionResponse,
    FurnessConfig,
    ImpedanceFunction,
    Link,
    LinkVolumesRequest,
    MacroServiceStub,
    ModelConfigRequest,
    ModeUtility,
    NetworkChunk,
    NewSessionRequest,
    Node,
    OdResultRequest,
    RegressionCoeffs,
    RunPipelineRequest,
    SessionId,
    TripGenerationConfig,
    UUIDv4,
    Zone,
    ZoneChunk,
)


def make_session_id(sid: str) -> UUIDv4:
    return UUIDv4(value=sid)


def main():
    addr = os.environ.get("MACRO_SIM_ADDR", "127.0.0.1:50052")
    channel = grpc.insecure_channel(addr)
    client = MacroServiceStub(channel)

    # Step 1: Create session
    resp = client.NewSession(NewSessionRequest())
    sid = resp.session_id.value
    print(f"Session created: {sid}")

    # Step 2: Push network (simple 5-node, 4-link chain)
    nodes = [
        Node(id=1, longitude=37.60, latitude=55.75, zone_id=1, macro_node_id=-1, macro_link_id=-1),
        Node(id=2, longitude=37.61, latitude=55.75, zone_id=-1, macro_node_id=-1, macro_link_id=-1),
        Node(id=3, longitude=37.62, latitude=55.75, zone_id=-1, macro_node_id=-1, macro_link_id=-1),
        Node(id=4, longitude=37.63, latitude=55.75, zone_id=-1, macro_node_id=-1, macro_link_id=-1),
        Node(id=5, longitude=37.64, latitude=55.75, zone_id=2, macro_node_id=-1, macro_link_id=-1),
    ]
    links = [
        Link(id=1, source_node_id=1, target_node_id=2, length_meters=500, free_speed=60, capacity=1800, lanes=2, link_type=3),
        Link(id=2, source_node_id=2, target_node_id=3, length_meters=500, free_speed=60, capacity=1800, lanes=2, link_type=3),
        Link(id=3, source_node_id=3, target_node_id=4, length_meters=500, free_speed=60, capacity=1800, lanes=2, link_type=3),
        Link(id=4, source_node_id=4, target_node_id=5, length_meters=500, free_speed=60, capacity=1800, lanes=2, link_type=3),
    ]

    def network_chunks():
        yield NetworkChunk(session_id=make_session_id(sid), nodes=nodes, links=links)

    for resp in client.PushNetwork(network_chunks()):
        print(f"Network: {resp.nodes_received} nodes, {resp.links_received} links ({resp.text})")

    # Step 3: Push zones
    zones = [
        Zone(id=1, name="Zone A", population=10000, employment=3000, households=4000, avg_income=50000, area_sq_km=2.5),
        Zone(id=2, name="Zone B", population=8000, employment=6000, households=3000, avg_income=45000, area_sq_km=3.0),
    ]

    def zone_chunks():
        yield ZoneChunk(session_id=make_session_id(sid), zones=zones)

    for resp in client.PushZones(zone_chunks()):
        print(f"Zones: {resp.zones_received} received ({resp.text})")

    # Step 4: Set model config
    config_resp = client.SetModelConfig(ModelConfigRequest(
        session_id=make_session_id(sid),
        assignment_method=0,  # Frank-Wolfe
        bpr=BprConfig(alpha=0.15, beta=4.0),
        assignment=AssignmentConvergence(max_iterations=50, convergence_gap=0.001),
        furness=FurnessConfig(max_iterations=100, tolerance=0.001),
        feedback_iterations=2,
        gp_step_scale=0.1,
        impedance=ImpedanceFunction(type=0, beta=0.1),  # Exponential
        trip_generation=TripGenerationConfig(
            method=0,  # Regression
            production_coeffs=RegressionCoeffs(intercept=0, pop_coeff=0.5, emp_coeff=0.1),
            attraction_coeffs=RegressionCoeffs(intercept=0, pop_coeff=0.1, emp_coeff=0.8),
        ),
        mode_utilities=[
            ModeUtility(mode="auto", asc=0.0, coeff_time=-0.03, coeff_distance=0.0, coeff_cost=-0.05),
            ModeUtility(mode="bike", asc=-1.5, coeff_time=-0.04, coeff_distance=-0.1, coeff_cost=0.0),
            ModeUtility(mode="walk", asc=-2.0, coeff_time=-0.05, coeff_distance=-0.2, coeff_cost=0.0),
        ],
    ))
    print(f"Config: {config_resp.code} ({config_resp.text})")

    # Step 5: Run pipeline
    print("\nRunning pipeline...")
    for progress in client.RunPipeline(RunPipelineRequest(session_id=make_session_id(sid))):
        if progress.is_failed:
            print(f"FAILED: {progress.error_message}")
            return
        print(
            f"  [period {progress.period_current}/{progress.period_total}] "
            f"phase={progress.phase}, "
            f"feedback={progress.feedback_current}/{progress.feedback_total}, "
            f"iter={progress.assignment_iteration}/{progress.assignment_max_iterations}, "
            f"gap={progress.relative_gap:.6f}"
            f"{' DONE' if progress.is_completed else ''}"
        )

    # Step 6: Get results
    print("\nLink volumes:")
    for chunk in client.GetLinkVolumes(LinkVolumesRequest(session_id=make_session_id(sid), period_index=0)):
        for v in chunk.volumes:
            print(f"  link {v.link_id}: volume={v.volume:.2f}, time={v.travel_time:.6f}")

    # Assignment info
    info_resp = client.GetAssignmentInfo(AssignmentInfoRequest(session_id=make_session_id(sid), period_index=0))
    if info_resp.info:
        ai = info_resp.info
        print(f"\nAssignment: {ai.iterations} iterations, gap={ai.relative_gap:.8f}, converged={ai.converged}")

    # Step 7: Cleanup
    del_resp = client.DeleteSession(SessionId(value=make_session_id(sid)))
    print(f"\nSession deleted: {del_resp.code} ({del_resp.text})")


if __name__ == "__main__":
    main()
