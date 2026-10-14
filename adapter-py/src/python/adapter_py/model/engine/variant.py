from __future__ import annotations
from pydantic import BaseModel, RootModel, WrapSerializer, BeforeValidator
from typing import Literal, Annotated
from adapter_py.model.helper import UintField, dict_serializer, dict_validator
from adapter_py.model.engine.geometry import Area, Coordinate
from adapter_py.model.engine.piece import PieceSimple
from adapter_py.model.engine.player import SeatId
from adapter_py.model.engine.tile import SpecialTilePair
from adapter_py.model.engine.faction import FactionId


class VariantData(BaseModel):
    board_width: UintField
    board_height: UintField
    promotion_area: Area
    initial_pieces: Annotated[
        dict[Coordinate, PieceSimple],
        BeforeValidator(lambda v: dict_validator(v, Coordinate)),
        WrapSerializer(dict_serializer),
    ]
    special_tile_pairs: list[SpecialTilePair]
    player_main_factions: dict[SeatId, FactionId]
    variant_name: str
    switch_prompt_enabled: bool
    strict_check: bool


class VariantPreset(RootModel):
    root: Literal["Standard"] | Literal["Arena"]


__all__ = ["VariantData", "VariantPreset"]
