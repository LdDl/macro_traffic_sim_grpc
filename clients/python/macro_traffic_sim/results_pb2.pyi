import uuid_pb2 as _uuid_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LinkVolumesRequest(_message.Message):
    __slots__ = ("session_id", "period_index")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    PERIOD_INDEX_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    period_index: int
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., period_index: _Optional[int] = ...) -> None: ...

class LinkVolume(_message.Message):
    __slots__ = ("link_id", "volume", "travel_time", "v_over_c")
    LINK_ID_FIELD_NUMBER: _ClassVar[int]
    VOLUME_FIELD_NUMBER: _ClassVar[int]
    TRAVEL_TIME_FIELD_NUMBER: _ClassVar[int]
    V_OVER_C_FIELD_NUMBER: _ClassVar[int]
    link_id: int
    volume: float
    travel_time: float
    v_over_c: float
    def __init__(self, link_id: _Optional[int] = ..., volume: _Optional[float] = ..., travel_time: _Optional[float] = ..., v_over_c: _Optional[float] = ...) -> None: ...

class LinkVolumesChunk(_message.Message):
    __slots__ = ("code", "text", "period_index", "volumes")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    PERIOD_INDEX_FIELD_NUMBER: _ClassVar[int]
    VOLUMES_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    period_index: int
    volumes: _containers.RepeatedCompositeFieldContainer[LinkVolume]
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., period_index: _Optional[int] = ..., volumes: _Optional[_Iterable[_Union[LinkVolume, _Mapping]]] = ...) -> None: ...

class SkimRequest(_message.Message):
    __slots__ = ("session_id", "mode", "skim_type")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    MODE_FIELD_NUMBER: _ClassVar[int]
    SKIM_TYPE_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    mode: str
    skim_type: str
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., mode: _Optional[str] = ..., skim_type: _Optional[str] = ...) -> None: ...

class SkimMatrixChunk(_message.Message):
    __slots__ = ("code", "text", "zone_ids", "data")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    ZONE_IDS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    zone_ids: _containers.RepeatedScalarFieldContainer[int]
    data: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., zone_ids: _Optional[_Iterable[int]] = ..., data: _Optional[_Iterable[float]] = ...) -> None: ...

class OdResultRequest(_message.Message):
    __slots__ = ("session_id", "mode")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    MODE_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    mode: str
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., mode: _Optional[str] = ...) -> None: ...

class OdResultChunk(_message.Message):
    __slots__ = ("code", "text", "zone_ids", "data")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    ZONE_IDS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    zone_ids: _containers.RepeatedScalarFieldContainer[int]
    data: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., zone_ids: _Optional[_Iterable[int]] = ..., data: _Optional[_Iterable[float]] = ...) -> None: ...

class AssignmentInfo(_message.Message):
    __slots__ = ("iterations", "relative_gap", "converged")
    ITERATIONS_FIELD_NUMBER: _ClassVar[int]
    RELATIVE_GAP_FIELD_NUMBER: _ClassVar[int]
    CONVERGED_FIELD_NUMBER: _ClassVar[int]
    iterations: int
    relative_gap: float
    converged: bool
    def __init__(self, iterations: _Optional[int] = ..., relative_gap: _Optional[float] = ..., converged: bool = ...) -> None: ...

class AssignmentInfoRequest(_message.Message):
    __slots__ = ("session_id", "period_index")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    PERIOD_INDEX_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    period_index: int
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., period_index: _Optional[int] = ...) -> None: ...

class AssignmentInfoResponse(_message.Message):
    __slots__ = ("code", "text", "info")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    INFO_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    info: AssignmentInfo
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., info: _Optional[_Union[AssignmentInfo, _Mapping]] = ...) -> None: ...
