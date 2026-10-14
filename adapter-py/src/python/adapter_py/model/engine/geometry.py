from __future__ import annotations
from pydantic import BaseModel, ConfigDict


class Coordinate(BaseModel):
    model_config = ConfigDict(frozen=True)
    row: int
    col: int


class Vec2(BaseModel):
    row: int
    col: int


class Area(BaseModel):
    left: int
    right: int
    top: int
    bottom: int


__all__ = ["Coordinate", "Vec2", "Area"]
