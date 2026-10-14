from adapter_py import rust_bridge  # pyright: ignore[reportAttributeAccessIssue]
from adapter_py.model.engine import VariantData, VariantPreset
from adapter_py.model.adapter import SessionAction, GameView
from typing import Self


class SessionAdapter:
    _inner: rust_bridge.SessionAdapter

    def __init__(self, inner: rust_bridge.SessionAdapter):
        self._inner = inner

    @classmethod
    def new(cls, variant_data: VariantData) -> Self:
        return cls(rust_bridge.SessionAdapter.new(variant_data.model_dump_json()))

    @classmethod
    def new_from_preset(cls, variant_preset: VariantPreset) -> Self:
        return cls(
            rust_bridge.SessionAdapter.new_from_preset(variant_preset.model_dump_json())
        )

    def do_action(self, action: SessionAction) -> bool:
        return self._inner.do_action(action.model_dump_json())

    def undo(self) -> bool:
        return self._inner.undo()

    def game_view(self) -> GameView:
        return GameView.model_validate_json(self._inner.game_view())
