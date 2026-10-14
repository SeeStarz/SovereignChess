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
    PINK = 1
    SLATE = 2
    RED = 3
    ORANGE = 4
    YELLOW = 5
    GREEN = 6
    CYAN = 7
    NAVY = 8
    ASH = 9
    VIOLET = 10
    BLACK = 11


class Allegiance(RootModel):
    root: Literal["Ally"] | Literal["Neutral"] | Literal["Enemy"]


__all__ = ["FactionId", "ColorStandard", "Allegiance"]
