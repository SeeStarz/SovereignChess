from __future__ import annotations
from pydantic import BaseModel, RootModel
from typing import Literal, Optional
from adapter_py.model.helper import EnumVariant
from adapter_py.model.engine.piece import PieceType, PieceRich
from adapter_py.model.engine.faction import FactionId
from adapter_py.model.engine.geometry import Coordinate
from adapter_py.model.engine.chess_move import MoveRich, CastleMoveRich


class Gesture(RootModel):
    root: GestureBoardVariant | GestureMenuClickVariant | GestureCancelVariant


class GestureBoardVariant(EnumVariant):
    board: BoardGesture


class GestureMenuClickVariant(EnumVariant):
    menu_click: MenuClick


class GestureCancelVariant(RootModel):
    root: Literal["Cancel"]


class MenuClick(RootModel):
    root: MenuClickPromotionVariant | MenuClickDefectionVariant | MenuClickCastleVariant


class MenuClickPromotionVariant(EnumVariant):
    promotion: PieceType


class MenuClickDefectionVariant(EnumVariant):
    defection: FactionId


class MenuClickCastleVariant(RootModel):
    root: Literal["Castle"]


class BoardGesture(RootModel):
    root: BoardGestureClickVariant | BoardGestureDropVariant


class BoardGestureClickVariant(EnumVariant):
    click: Coordinate


class BoardGestureDropVariant(EnumVariant):
    drop: Optional[Coordinate]


class IntraGameStateStage(RootModel):
    root: Literal["Idle"] | Literal["Selected"] | Literal["Promotion"] | Literal["Done"]


class IntraGameStateView(BaseModel):
    grabbable_pieces: list[PieceRich]
    valid_defections: list[FactionId]
    valid_castles: list[CastleMoveRich]
    grabbed_piece: Optional[PieceRich]
    selected_defection: Optional[FactionId]
    castle_selected: bool
    valid_destinations: list[Coordinate]
    promotion_options: list[PieceType]
    move_made: Optional[MoveRich]
    state: IntraGameStateStage


__all__ = [
    "Gesture",
    "GestureBoardVariant",
    "GestureMenuClickVariant",
    "GestureCancelVariant",
    "MenuClick",
    "MenuClickPromotionVariant",
    "MenuClickDefectionVariant",
    "MenuClickCastleVariant",
    "BoardGesture",
    "BoardGestureClickVariant",
    "BoardGestureDropVariant",
    "IntraGameStateStage",
    "IntraGameStateView",
]
