import uuid_pb2 as _uuid_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AssignmentMethod(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    ASSIGNMENT_FRANK_WOLFE: _ClassVar[AssignmentMethod]
    ASSIGNMENT_MSA: _ClassVar[AssignmentMethod]
    ASSIGNMENT_GRADIENT_PROJECTION: _ClassVar[AssignmentMethod]

class ImpedanceFunctionType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    IMPEDANCE_EXPONENTIAL: _ClassVar[ImpedanceFunctionType]
    IMPEDANCE_POWER: _ClassVar[ImpedanceFunctionType]
    IMPEDANCE_COMBINED: _ClassVar[ImpedanceFunctionType]

class TripGenerationMethod(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    TRIP_GEN_REGRESSION: _ClassVar[TripGenerationMethod]
    TRIP_GEN_CROSS_CLASSIFICATION: _ClassVar[TripGenerationMethod]
ASSIGNMENT_FRANK_WOLFE: AssignmentMethod
ASSIGNMENT_MSA: AssignmentMethod
ASSIGNMENT_GRADIENT_PROJECTION: AssignmentMethod
IMPEDANCE_EXPONENTIAL: ImpedanceFunctionType
IMPEDANCE_POWER: ImpedanceFunctionType
IMPEDANCE_COMBINED: ImpedanceFunctionType
TRIP_GEN_REGRESSION: TripGenerationMethod
TRIP_GEN_CROSS_CLASSIFICATION: TripGenerationMethod

class BprConfig(_message.Message):
    __slots__ = ("alpha", "beta")
    ALPHA_FIELD_NUMBER: _ClassVar[int]
    BETA_FIELD_NUMBER: _ClassVar[int]
    alpha: float
    beta: float
    def __init__(self, alpha: _Optional[float] = ..., beta: _Optional[float] = ...) -> None: ...

class AssignmentConvergence(_message.Message):
    __slots__ = ("max_iterations", "convergence_gap")
    MAX_ITERATIONS_FIELD_NUMBER: _ClassVar[int]
    CONVERGENCE_GAP_FIELD_NUMBER: _ClassVar[int]
    max_iterations: int
    convergence_gap: float
    def __init__(self, max_iterations: _Optional[int] = ..., convergence_gap: _Optional[float] = ...) -> None: ...

class FurnessConfig(_message.Message):
    __slots__ = ("max_iterations", "tolerance")
    MAX_ITERATIONS_FIELD_NUMBER: _ClassVar[int]
    TOLERANCE_FIELD_NUMBER: _ClassVar[int]
    max_iterations: int
    tolerance: float
    def __init__(self, max_iterations: _Optional[int] = ..., tolerance: _Optional[float] = ...) -> None: ...

class ImpedanceFunction(_message.Message):
    __slots__ = ("type", "alpha", "beta")
    TYPE_FIELD_NUMBER: _ClassVar[int]
    ALPHA_FIELD_NUMBER: _ClassVar[int]
    BETA_FIELD_NUMBER: _ClassVar[int]
    type: ImpedanceFunctionType
    alpha: float
    beta: float
    def __init__(self, type: _Optional[_Union[ImpedanceFunctionType, str]] = ..., alpha: _Optional[float] = ..., beta: _Optional[float] = ...) -> None: ...

class RegressionCoeffs(_message.Message):
    __slots__ = ("intercept", "pop_coeff", "emp_coeff", "hh_coeff", "income_coeff")
    INTERCEPT_FIELD_NUMBER: _ClassVar[int]
    POP_COEFF_FIELD_NUMBER: _ClassVar[int]
    EMP_COEFF_FIELD_NUMBER: _ClassVar[int]
    HH_COEFF_FIELD_NUMBER: _ClassVar[int]
    INCOME_COEFF_FIELD_NUMBER: _ClassVar[int]
    intercept: float
    pop_coeff: float
    emp_coeff: float
    hh_coeff: float
    income_coeff: float
    def __init__(self, intercept: _Optional[float] = ..., pop_coeff: _Optional[float] = ..., emp_coeff: _Optional[float] = ..., hh_coeff: _Optional[float] = ..., income_coeff: _Optional[float] = ...) -> None: ...

class TripGenerationConfig(_message.Message):
    __slots__ = ("method", "production_coeffs", "attraction_coeffs", "attraction_rate_per_employee", "hh_size_thresholds", "income_thresholds", "production_rates")
    class ProductionRatesEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: float
        def __init__(self, key: _Optional[str] = ..., value: _Optional[float] = ...) -> None: ...
    METHOD_FIELD_NUMBER: _ClassVar[int]
    PRODUCTION_COEFFS_FIELD_NUMBER: _ClassVar[int]
    ATTRACTION_COEFFS_FIELD_NUMBER: _ClassVar[int]
    ATTRACTION_RATE_PER_EMPLOYEE_FIELD_NUMBER: _ClassVar[int]
    HH_SIZE_THRESHOLDS_FIELD_NUMBER: _ClassVar[int]
    INCOME_THRESHOLDS_FIELD_NUMBER: _ClassVar[int]
    PRODUCTION_RATES_FIELD_NUMBER: _ClassVar[int]
    method: TripGenerationMethod
    production_coeffs: RegressionCoeffs
    attraction_coeffs: RegressionCoeffs
    attraction_rate_per_employee: float
    hh_size_thresholds: _containers.RepeatedScalarFieldContainer[float]
    income_thresholds: _containers.RepeatedScalarFieldContainer[float]
    production_rates: _containers.ScalarMap[str, float]
    def __init__(self, method: _Optional[_Union[TripGenerationMethod, str]] = ..., production_coeffs: _Optional[_Union[RegressionCoeffs, _Mapping]] = ..., attraction_coeffs: _Optional[_Union[RegressionCoeffs, _Mapping]] = ..., attraction_rate_per_employee: _Optional[float] = ..., hh_size_thresholds: _Optional[_Iterable[float]] = ..., income_thresholds: _Optional[_Iterable[float]] = ..., production_rates: _Optional[_Mapping[str, float]] = ...) -> None: ...

class ModeUtility(_message.Message):
    __slots__ = ("mode", "asc", "coeff_time", "coeff_distance", "coeff_cost")
    MODE_FIELD_NUMBER: _ClassVar[int]
    ASC_FIELD_NUMBER: _ClassVar[int]
    COEFF_TIME_FIELD_NUMBER: _ClassVar[int]
    COEFF_DISTANCE_FIELD_NUMBER: _ClassVar[int]
    COEFF_COST_FIELD_NUMBER: _ClassVar[int]
    mode: str
    asc: float
    coeff_time: float
    coeff_distance: float
    coeff_cost: float
    def __init__(self, mode: _Optional[str] = ..., asc: _Optional[float] = ..., coeff_time: _Optional[float] = ..., coeff_distance: _Optional[float] = ..., coeff_cost: _Optional[float] = ...) -> None: ...

class TimePeriod(_message.Message):
    __slots__ = ("name", "start_hour", "end_hour", "demand_factor")
    NAME_FIELD_NUMBER: _ClassVar[int]
    START_HOUR_FIELD_NUMBER: _ClassVar[int]
    END_HOUR_FIELD_NUMBER: _ClassVar[int]
    DEMAND_FACTOR_FIELD_NUMBER: _ClassVar[int]
    name: str
    start_hour: float
    end_hour: float
    demand_factor: float
    def __init__(self, name: _Optional[str] = ..., start_hour: _Optional[float] = ..., end_hour: _Optional[float] = ..., demand_factor: _Optional[float] = ...) -> None: ...

class ModelConfigRequest(_message.Message):
    __slots__ = ("session_id", "assignment_method", "bpr", "assignment", "furness", "feedback_iterations", "gp_step_scale", "impedance", "trip_generation", "mode_utilities", "time_periods")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    ASSIGNMENT_METHOD_FIELD_NUMBER: _ClassVar[int]
    BPR_FIELD_NUMBER: _ClassVar[int]
    ASSIGNMENT_FIELD_NUMBER: _ClassVar[int]
    FURNESS_FIELD_NUMBER: _ClassVar[int]
    FEEDBACK_ITERATIONS_FIELD_NUMBER: _ClassVar[int]
    GP_STEP_SCALE_FIELD_NUMBER: _ClassVar[int]
    IMPEDANCE_FIELD_NUMBER: _ClassVar[int]
    TRIP_GENERATION_FIELD_NUMBER: _ClassVar[int]
    MODE_UTILITIES_FIELD_NUMBER: _ClassVar[int]
    TIME_PERIODS_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    assignment_method: AssignmentMethod
    bpr: BprConfig
    assignment: AssignmentConvergence
    furness: FurnessConfig
    feedback_iterations: int
    gp_step_scale: float
    impedance: ImpedanceFunction
    trip_generation: TripGenerationConfig
    mode_utilities: _containers.RepeatedCompositeFieldContainer[ModeUtility]
    time_periods: _containers.RepeatedCompositeFieldContainer[TimePeriod]
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., assignment_method: _Optional[_Union[AssignmentMethod, str]] = ..., bpr: _Optional[_Union[BprConfig, _Mapping]] = ..., assignment: _Optional[_Union[AssignmentConvergence, _Mapping]] = ..., furness: _Optional[_Union[FurnessConfig, _Mapping]] = ..., feedback_iterations: _Optional[int] = ..., gp_step_scale: _Optional[float] = ..., impedance: _Optional[_Union[ImpedanceFunction, _Mapping]] = ..., trip_generation: _Optional[_Union[TripGenerationConfig, _Mapping]] = ..., mode_utilities: _Optional[_Iterable[_Union[ModeUtility, _Mapping]]] = ..., time_periods: _Optional[_Iterable[_Union[TimePeriod, _Mapping]]] = ...) -> None: ...

class ModelConfigResponse(_message.Message):
    __slots__ = ("code", "text")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ...) -> None: ...
