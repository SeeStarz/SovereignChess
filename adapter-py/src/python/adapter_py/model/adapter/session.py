from __future__ import annotations
from pydantic import BaseModel, RootModel
from adapter_py.model.helper import EnumVariant
from adapter_py.model.adapter.gesture import Gesture, IntraGameStateView
from adapter_py.model.engine.chess_move import MoveSimple
from adapter_py.model.engine.game_state_view import GameStateView


class SessionAction(RootModel):
    root: (
        SessionActionMoveVariant
        | SessionActionGestureVariant
        | SessionActionSwapVariant
    )


class SessionActionMoveVariant(EnumVariant):
    move: MoveSimple


class SessionActionGestureVariant(EnumVariant):
    gesture: Gesture


class SessionActionSwapVariant(EnumVariant):
    swap: bool


class GameView(BaseModel):
    intra_game_state: IntraGameStateView
    game_state: GameStateView


__all__ = [
    "SessionAction",
    "SessionActionMoveVariant",
    "SessionActionGestureVariant",
    "SessionActionSwapVariant",
    "GameView",
]
