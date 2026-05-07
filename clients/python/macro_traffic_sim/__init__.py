"""
macro_traffic_sim - Python gRPC client for macroscopic traffic simulation.

Usage:
    import grpc
    from macro_traffic_sim import (
        MacroServiceStub,
        NewSessionRequest,
        Node,
        Link,
        Zone,
    )

    channel = grpc.insecure_channel("127.0.0.1:50052")
    client = MacroServiceStub(channel)

    # Create session
    response = client.NewSession(NewSessionRequest())
    session_id = response.session_id.value
"""

from .uuid_pb2 import UUIDv4
from .session_pb2 import (
    NewSessionRequest,
    NewSessionResponse,
    SessionId,
    InfoSessionResponse,
    DeleteSessionResponse,
)
from .network_pb2 import (
    Node,
    Link,
    LinkType,
    Zone,
    NetworkChunk,
    NetworkChunkResponse,
    ZoneChunk,
    ZoneChunkResponse,
    OdMatrixChunk,
    OdMatrixChunkResponse,
)
from .config_pb2 import (
    AssignmentMethod,
    ImpedanceFunctionType,
    TripGenerationMethod,
    BprConfig,
    AssignmentConvergence,
    FurnessConfig,
    ImpedanceFunction,
    RegressionCoeffs,
    TripGenerationConfig,
    ModeUtility,
    TimePeriod,
    ModelConfigRequest,
    ModelConfigResponse,
)
from .run_pb2 import (
    RunPipelineRequest,
    RunPipelineProgress,
)
from .results_pb2 import (
    LinkVolumesRequest,
    LinkVolume,
    LinkVolumesChunk,
    SkimRequest,
    SkimMatrixChunk,
    OdResultRequest,
    OdResultChunk,
    AssignmentInfo,
    AssignmentInfoRequest,
    AssignmentInfoResponse,
)
from .service_pb2_grpc import MacroServiceStub

__all__ = [
    # UUID
    "UUIDv4",
    # Session
    "NewSessionRequest",
    "NewSessionResponse",
    "SessionId",
    "InfoSessionResponse",
    "DeleteSessionResponse",
    # Network
    "Node",
    "Link",
    "LinkType",
    "Zone",
    "NetworkChunk",
    "NetworkChunkResponse",
    "ZoneChunk",
    "ZoneChunkResponse",
    "OdMatrixChunk",
    "OdMatrixChunkResponse",
    # Config
    "AssignmentMethod",
    "ImpedanceFunctionType",
    "TripGenerationMethod",
    "BprConfig",
    "AssignmentConvergence",
    "FurnessConfig",
    "ImpedanceFunction",
    "RegressionCoeffs",
    "TripGenerationConfig",
    "ModeUtility",
    "TimePeriod",
    "ModelConfigRequest",
    "ModelConfigResponse",
    # Run
    "RunPipelineRequest",
    "RunPipelineProgress",
    # Results
    "LinkVolumesRequest",
    "LinkVolume",
    "LinkVolumesChunk",
    "SkimRequest",
    "SkimMatrixChunk",
    "OdResultRequest",
    "OdResultChunk",
    "AssignmentInfo",
    "AssignmentInfoRequest",
    "AssignmentInfoResponse",
    # Service
    "MacroServiceStub",
]

__version__ = "0.1.0"
