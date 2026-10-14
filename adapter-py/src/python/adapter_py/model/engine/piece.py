from __future__ import annotations
from pydantic import BaseModel, RootModel
from typing import Literal, Optional
from adapter_py.model.engine.faction import FactionId
from adapter_py.model.engine.geometry import Coordinate


class PieceType(RootModel):
    root: (
        Literal["Pawn"]
        | Literal["Knight"]
        | Literal["Bishop"]
        | Literal["Rook"]
        | Literal["Queen"]
        | Literal["King"]
    )


class PieceSimple(BaseModel):
    faction: FactionId
    piece_type: PieceType


class PieceRich(BaseModel):
    faction: FactionId
    owner: Optional[FactionId]
    piece_type: PieceType
    coordinate: Coordinate


__all__ = ["PieceType", "PieceSimple", "PieceRich"]
