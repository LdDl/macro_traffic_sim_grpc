import uuid_pb2 as _uuid_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class NewSessionRequest(_message.Message):
    __slots__ = ()
    def __init__(self) -> None: ...

class NewSessionResponse(_message.Message):
    __slots__ = ("code", "text", "session_id")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    session_id: _uuid_pb2.UUIDv4
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ...) -> None: ...

class SessionId(_message.Message):
    __slots__ = ("value",)
    VALUE_FIELD_NUMBER: _ClassVar[int]
    value: _uuid_pb2.UUIDv4
    def __init__(self, value: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ...) -> None: ...

class InfoSessionResponse(_message.Message):
    __slots__ = ("code", "text", "session_id", "state", "has_network", "has_zones", "has_od_matrix", "has_config", "has_results", "num_nodes", "num_links", "num_zones")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    HAS_NETWORK_FIELD_NUMBER: _ClassVar[int]
    HAS_ZONES_FIELD_NUMBER: _ClassVar[int]
    HAS_OD_MATRIX_FIELD_NUMBER: _ClassVar[int]
    HAS_CONFIG_FIELD_NUMBER: _ClassVar[int]
    HAS_RESULTS_FIELD_NUMBER: _ClassVar[int]
    NUM_NODES_FIELD_NUMBER: _ClassVar[int]
    NUM_LINKS_FIELD_NUMBER: _ClassVar[int]
    NUM_ZONES_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    session_id: _uuid_pb2.UUIDv4
    state: str
    has_network: bool
    has_zones: bool
    has_od_matrix: bool
    has_config: bool
    has_results: bool
    num_nodes: int
    num_links: int
    num_zones: int
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., state: _Optional[str] = ..., has_network: bool = ..., has_zones: bool = ..., has_od_matrix: bool = ..., has_config: bool = ..., has_results: bool = ..., num_nodes: _Optional[int] = ..., num_links: _Optional[int] = ..., num_zones: _Optional[int] = ...) -> None: ...

class DeleteSessionResponse(_message.Message):
    __slots__ = ("code", "text")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ...) -> None: ...
