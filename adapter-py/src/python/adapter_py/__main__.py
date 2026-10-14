from adapter_py.model.engine import VariantPreset
from adapter_py.api.session import SessionAdapter

if __name__ == "__main__":
    adapter: SessionAdapter = SessionAdapter.new_from_preset(
        VariantPreset(root="Standard")
    )
    print(adapter.game_view())
