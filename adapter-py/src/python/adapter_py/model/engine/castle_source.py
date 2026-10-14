from __future__ import annotations
from pydantic import BaseModel
from adapter_py.model.engine.geometry import Coordinate


class CastleSource(BaseModel):
    king_coordinate: Coordinate
    rook_coordinate: Coordinate


__all__ = ["CastleSource"]
