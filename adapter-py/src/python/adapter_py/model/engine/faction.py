from __future__ import annotations
from pydantic import RootModel, ConfigDict
from typing import Literal
from enum import Enum
from adapter_py.model.helper import UintField


class FactionId(RootModel):
    model_config = ConfigDict(frozen=True)
    root: UintField


class ColorStandard(Enum):
    WHITE = 0
    ASH = 1
    SLATE = 2
    BLACK = 3
    PINK = 4
    RED = 5
    ORANGE = 6
    YELLOW = 7
    GREEN = 8
    NAVY = 9
    CYAN = 10
    VIOLET = 11


class Allegiance(RootModel):
    root: Literal["Ally"] | Literal["Neutral"] | Literal["Enemy"]


__all__ = ["FactionId", "ColorStandard", "Allegiance"]
