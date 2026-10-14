from __future__ import annotations
from pydantic import BaseModel, RootModel
from adapter_py.model.helper import UintField
from adapter_py.model.engine.geometry import Area
from adapter_py.model.engine.tile import TileRich, SpecialTilePair
from adapter_py.model.engine.piece import PieceRich


class BoardView(BaseModel):
    rows: list[BoardRow]
    tiles: list[TileRich]
    pieces: list[PieceRich]
    special_tile_pairs: list[SpecialTilePair]
    promotion_area: Area
    width: UintField
    height: UintField


class BoardRow(RootModel):
    root: list[TileRich]


__all__ = ["BoardView", "BoardRow"]
