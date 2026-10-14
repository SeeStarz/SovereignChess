from __future__ import annotations
from pydantic import BaseModel
from typing import Optional
from adapter_py.model.engine.faction import FactionId
from adapter_py.model.engine.geometry import Coordinate
from adapter_py.model.engine.piece import PieceRich


class TileRich(BaseModel):
    piece: Optional[PieceRich]
    special: Optional[SpecialTilePair]
    coordinate: Coordinate


class SpecialTilePair(BaseModel):
    faction: FactionId
    coordinates: tuple[Coordinate, Coordinate]


__all__ = ["TileRich", "SpecialTilePair"]
