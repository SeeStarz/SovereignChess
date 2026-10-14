from __future__ import annotations
from pydantic import BaseModel
from adapter_py.model.helper import UintField
from adapter_py.model.engine.board_view import BoardView
from adapter_py.model.engine.player import PlayerId
from adapter_py.model.engine.faction import FactionId
from adapter_py.model.engine.variant import VariantData


class GameStateView(BaseModel):
    board_view: BoardView
    player_main_factions: dict[PlayerId, FactionId]
    player_owned_factions: dict[PlayerId, FactionId]
    owned_factions: dict[FactionId, FactionId]
    factions: list[FactionId]
    turn_to_play: PlayerId
    player_count: UintField
    waiting_swap: bool
    variant_data: VariantData


__all__ = ["GameStateView"]
