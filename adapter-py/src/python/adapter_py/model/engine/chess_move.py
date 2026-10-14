from __future__ import annotations
from pydantic import BaseModel, RootModel
from typing import Optional
from adapter_py.model.helper import EnumVariant
from adapter_py.model.engine.geometry import Coordinate
from adapter_py.model.engine.piece import PieceType
from adapter_py.model.engine.faction import FactionId


class MoveSimple(RootModel):
    root: (
        MoveSimpleNormalMoveVariant
        | MoveSimpleCastleVariant
        | MoveSimplePromotionVariant
        | MoveSimpleRegimeChangePromotionVariant
        | MoveSimpleDefectionVariant
    )


class MoveSimpleNormalMoveVariant(EnumVariant):
    normal_move: NormalMove


class MoveSimpleCastleVariant(EnumVariant):
    castle: CastleMoveSimple


class MoveSimplePromotionVariant(EnumVariant):
    promotion: PromotionMove


class MoveSimpleRegimeChangePromotionVariant(EnumVariant):
    regime_change_promotion_simple: RegimeChangePromotionMoveSimple


class MoveSimpleDefectionVariant(EnumVariant):
    defection: DefectionMoveSimple


class MoveRich(RootModel):
    root: (
        MoveRichNormalMoveVariant
        | MoveRichCastleVariant
        | MoveRichPromotionVariant
        | MoveRichRegimeChangePromotionVariant
        | MoveRichDefectionVariant
    )


class MoveRichNormalMoveVariant(EnumVariant):
    normal_move: NormalMove


class MoveRichCastleVariant(EnumVariant):
    castle: CastleMoveRich


class MoveRichPromotionVariant(EnumVariant):
    promotion: PromotionMove


class MoveRichRegimeChangePromotionVariant(EnumVariant):
    regime_change_promotion_rich: RegimeChangePromotionMoveRich


class MoveRichDefectionVariant(EnumVariant):
    defection: DefectionMoveRich


class NormalMove(BaseModel):
    origin: Coordinate
    destination: Coordinate


class CastleMoveSimple(BaseModel):
    king_destination: Coordinate
    rook_move: NormalMove


class CastleMoveRich(BaseModel):
    king_move: NormalMove
    rook_move: NormalMove


class PromotionMove(BaseModel):
    normal_move: NormalMove
    piece_type: PieceType


class RegimeChangePromotionMoveSimple(BaseModel):
    pawn_move: NormalMove


class RegimeChangePromotionMoveRich(BaseModel):
    pawn_move: NormalMove
    king_origin: Optional[Coordinate]


class DefectionMoveSimple(BaseModel):
    destination: Optional[Coordinate]
    faction: FactionId


class DefectionMoveRich(BaseModel):
    origin: Coordinate
    destination: Optional[Coordinate]
    faction: FactionId


__all__ = [
    "MoveSimple",
    "MoveSimpleNormalMoveVariant",
    "MoveSimpleCastleVariant",
    "MoveSimplePromotionVariant",
    "MoveSimpleRegimeChangePromotionVariant",
    "MoveSimpleDefectionVariant",
    "MoveRich",
    "MoveRichNormalMoveVariant",
    "MoveRichCastleVariant",
    "MoveRichPromotionVariant",
    "MoveRichRegimeChangePromotionVariant",
    "MoveRichDefectionVariant",
    "NormalMove",
    "CastleMoveSimple",
    "CastleMoveRich",
    "PromotionMove",
    "RegimeChangePromotionMoveSimple",
    "RegimeChangePromotionMoveRich",
    "DefectionMoveSimple",
    "DefectionMoveRich",
]
