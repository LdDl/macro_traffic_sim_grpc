import uuid_pb2 as _uuid_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class RunPipelineRequest(_message.Message):
    __slots__ = ("session_id",)
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ...) -> None: ...

class RunPipelineProgress(_message.Message):
    __slots__ = ("code", "text", "session_id", "phase", "period_current", "period_total", "feedback_current", "feedback_total", "assignment_iteration", "assignment_max_iterations", "relative_gap", "is_completed", "is_failed", "error_message")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    PHASE_FIELD_NUMBER: _ClassVar[int]
    PERIOD_CURRENT_FIELD_NUMBER: _ClassVar[int]
    PERIOD_TOTAL_FIELD_NUMBER: _ClassVar[int]
    FEEDBACK_CURRENT_FIELD_NUMBER: _ClassVar[int]
    FEEDBACK_TOTAL_FIELD_NUMBER: _ClassVar[int]
    ASSIGNMENT_ITERATION_FIELD_NUMBER: _ClassVar[int]
    ASSIGNMENT_MAX_ITERATIONS_FIELD_NUMBER: _ClassVar[int]
    RELATIVE_GAP_FIELD_NUMBER: _ClassVar[int]
    IS_COMPLETED_FIELD_NUMBER: _ClassVar[int]
    IS_FAILED_FIELD_NUMBER: _ClassVar[int]
    ERROR_MESSAGE_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    session_id: _uuid_pb2.UUIDv4
    phase: str
    period_current: int
    period_total: int
    feedback_current: int
    feedback_total: int
    assignment_iteration: int
    assignment_max_iterations: int
    relative_gap: float
    is_completed: bool
    is_failed: bool
    error_message: str
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., phase: _Optional[str] = ..., period_current: _Optional[int] = ..., period_total: _Optional[int] = ..., feedback_current: _Optional[int] = ..., feedback_total: _Optional[int] = ..., assignment_iteration: _Optional[int] = ..., assignment_max_iterations: _Optional[int] = ..., relative_gap: _Optional[float] = ..., is_completed: bool = ..., is_failed: bool = ..., error_message: _Optional[str] = ...) -> None: ...
