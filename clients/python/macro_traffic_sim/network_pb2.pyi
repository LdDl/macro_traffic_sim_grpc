import uuid_pb2 as _uuid_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LinkType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    LINK_TYPE_UNDEFINED: _ClassVar[LinkType]
    LINK_TYPE_MOTORWAY: _ClassVar[LinkType]
    LINK_TYPE_TRUNK: _ClassVar[LinkType]
    LINK_TYPE_PRIMARY: _ClassVar[LinkType]
    LINK_TYPE_SECONDARY: _ClassVar[LinkType]
    LINK_TYPE_TERTIARY: _ClassVar[LinkType]
    LINK_TYPE_RESIDENTIAL: _ClassVar[LinkType]
    LINK_TYPE_LIVING_STREET: _ClassVar[LinkType]
    LINK_TYPE_SERVICE: _ClassVar[LinkType]
    LINK_TYPE_CYCLEWAY: _ClassVar[LinkType]
    LINK_TYPE_FOOTWAY: _ClassVar[LinkType]
    LINK_TYPE_TRACK: _ClassVar[LinkType]
    LINK_TYPE_UNCLASSIFIED: _ClassVar[LinkType]
    LINK_TYPE_CONNECTOR: _ClassVar[LinkType]
    LINK_TYPE_RAILWAY: _ClassVar[LinkType]
    LINK_TYPE_AEROWAY: _ClassVar[LinkType]
LINK_TYPE_UNDEFINED: LinkType
LINK_TYPE_MOTORWAY: LinkType
LINK_TYPE_TRUNK: LinkType
LINK_TYPE_PRIMARY: LinkType
LINK_TYPE_SECONDARY: LinkType
LINK_TYPE_TERTIARY: LinkType
LINK_TYPE_RESIDENTIAL: LinkType
LINK_TYPE_LIVING_STREET: LinkType
LINK_TYPE_SERVICE: LinkType
LINK_TYPE_CYCLEWAY: LinkType
LINK_TYPE_FOOTWAY: LinkType
LINK_TYPE_TRACK: LinkType
LINK_TYPE_UNCLASSIFIED: LinkType
LINK_TYPE_CONNECTOR: LinkType
LINK_TYPE_RAILWAY: LinkType
LINK_TYPE_AEROWAY: LinkType

class Node(_message.Message):
    __slots__ = ("id", "longitude", "latitude", "zone_id", "macro_node_id", "macro_link_id")
    ID_FIELD_NUMBER: _ClassVar[int]
    LONGITUDE_FIELD_NUMBER: _ClassVar[int]
    LATITUDE_FIELD_NUMBER: _ClassVar[int]
    ZONE_ID_FIELD_NUMBER: _ClassVar[int]
    MACRO_NODE_ID_FIELD_NUMBER: _ClassVar[int]
    MACRO_LINK_ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    longitude: float
    latitude: float
    zone_id: int
    macro_node_id: int
    macro_link_id: int
    def __init__(self, id: _Optional[int] = ..., longitude: _Optional[float] = ..., latitude: _Optional[float] = ..., zone_id: _Optional[int] = ..., macro_node_id: _Optional[int] = ..., macro_link_id: _Optional[int] = ...) -> None: ...

class Link(_message.Message):
    __slots__ = ("id", "source_node_id", "target_node_id", "length_meters", "free_speed", "capacity", "lanes", "link_type", "is_connection", "macro_link_id", "movement_id")
    ID_FIELD_NUMBER: _ClassVar[int]
    SOURCE_NODE_ID_FIELD_NUMBER: _ClassVar[int]
    TARGET_NODE_ID_FIELD_NUMBER: _ClassVar[int]
    LENGTH_METERS_FIELD_NUMBER: _ClassVar[int]
    FREE_SPEED_FIELD_NUMBER: _ClassVar[int]
    CAPACITY_FIELD_NUMBER: _ClassVar[int]
    LANES_FIELD_NUMBER: _ClassVar[int]
    LINK_TYPE_FIELD_NUMBER: _ClassVar[int]
    IS_CONNECTION_FIELD_NUMBER: _ClassVar[int]
    MACRO_LINK_ID_FIELD_NUMBER: _ClassVar[int]
    MOVEMENT_ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    source_node_id: int
    target_node_id: int
    length_meters: float
    free_speed: float
    capacity: float
    lanes: int
    link_type: LinkType
    is_connection: bool
    macro_link_id: int
    movement_id: int
    def __init__(self, id: _Optional[int] = ..., source_node_id: _Optional[int] = ..., target_node_id: _Optional[int] = ..., length_meters: _Optional[float] = ..., free_speed: _Optional[float] = ..., capacity: _Optional[float] = ..., lanes: _Optional[int] = ..., link_type: _Optional[_Union[LinkType, str]] = ..., is_connection: bool = ..., macro_link_id: _Optional[int] = ..., movement_id: _Optional[int] = ...) -> None: ...

class Zone(_message.Message):
    __slots__ = ("id", "name", "population", "employment", "households", "avg_income", "area_sq_km")
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    POPULATION_FIELD_NUMBER: _ClassVar[int]
    EMPLOYMENT_FIELD_NUMBER: _ClassVar[int]
    HOUSEHOLDS_FIELD_NUMBER: _ClassVar[int]
    AVG_INCOME_FIELD_NUMBER: _ClassVar[int]
    AREA_SQ_KM_FIELD_NUMBER: _ClassVar[int]
    id: int
    name: str
    population: float
    employment: float
    households: float
    avg_income: float
    area_sq_km: float
    def __init__(self, id: _Optional[int] = ..., name: _Optional[str] = ..., population: _Optional[float] = ..., employment: _Optional[float] = ..., households: _Optional[float] = ..., avg_income: _Optional[float] = ..., area_sq_km: _Optional[float] = ...) -> None: ...

class NetworkChunk(_message.Message):
    __slots__ = ("session_id", "nodes", "links")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    NODES_FIELD_NUMBER: _ClassVar[int]
    LINKS_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    nodes: _containers.RepeatedCompositeFieldContainer[Node]
    links: _containers.RepeatedCompositeFieldContainer[Link]
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., nodes: _Optional[_Iterable[_Union[Node, _Mapping]]] = ..., links: _Optional[_Iterable[_Union[Link, _Mapping]]] = ...) -> None: ...

class NetworkChunkResponse(_message.Message):
    __slots__ = ("code", "text", "nodes_received", "links_received")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    NODES_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    LINKS_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    nodes_received: int
    links_received: int
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., nodes_received: _Optional[int] = ..., links_received: _Optional[int] = ...) -> None: ...

class ZoneChunk(_message.Message):
    __slots__ = ("session_id", "zones")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    ZONES_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    zones: _containers.RepeatedCompositeFieldContainer[Zone]
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., zones: _Optional[_Iterable[_Union[Zone, _Mapping]]] = ...) -> None: ...

class ZoneChunkResponse(_message.Message):
    __slots__ = ("code", "text", "zones_received")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    ZONES_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    zones_received: int
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., zones_received: _Optional[int] = ...) -> None: ...

class OdMatrixChunk(_message.Message):
    __slots__ = ("session_id", "zone_ids", "data")
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    ZONE_IDS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    session_id: _uuid_pb2.UUIDv4
    zone_ids: _containers.RepeatedScalarFieldContainer[int]
    data: _containers.RepeatedScalarFieldContainer[float]
    def __init__(self, session_id: _Optional[_Union[_uuid_pb2.UUIDv4, _Mapping]] = ..., zone_ids: _Optional[_Iterable[int]] = ..., data: _Optional[_Iterable[float]] = ...) -> None: ...

class OdMatrixChunkResponse(_message.Message):
    __slots__ = ("code", "text", "cells_received")
    CODE_FIELD_NUMBER: _ClassVar[int]
    TEXT_FIELD_NUMBER: _ClassVar[int]
    CELLS_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    code: int
    text: str
    cells_received: int
    def __init__(self, code: _Optional[int] = ..., text: _Optional[str] = ..., cells_received: _Optional[int] = ...) -> None: ...
