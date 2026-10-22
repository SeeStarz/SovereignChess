from __future__ import annotations
from pydantic import BaseModel, RootModel, ConfigDict
from typing import Literal, Optional, Self
from adapter_py.model.engine.faction import FactionId
from adapter_py.model.engine.geometry import Coordinate


class PieceType(RootModel):
    model_config: ConfigDict = ConfigDict(frozen=True)
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

    @classmethod
    def from_rich(cls, piece: PieceRich) -> Self:
        return cls(faction=piece.faction, piece_type=piece.piece_type)


class PieceRich(BaseModel):
    faction: FactionId
    owner: Optional[FactionId]
    piece_type: PieceType
    coordinate: Coordinate


__all__ = ["PieceType", "PieceSimple", "PieceRich"]
