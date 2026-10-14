from __future__ import annotations
from pydantic import RootModel, ConfigDict
from adapter_py.model.helper import UintField


class PlayerId(RootModel):
    model_config = ConfigDict(frozen=True)
    root: UintField


class SeatId(RootModel):
    model_config = ConfigDict(frozen=True)
    root: UintField


__all__ = ["PlayerId", "SeatId"]
