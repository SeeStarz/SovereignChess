use adapter_core::SessionAdapter;
use shared_model::definition::engine::{VariantData, VariantPreset};

#[test]
fn serialize_game_view() {
    serde_json::to_string(&SessionAdapter::new_from_preset(VariantPreset::Standard).game_view())
        .expect("Failed to serialize GameView");
}

#[test]
fn serialize_variant_data() {
    serde_json::to_string(&VariantData::from(VariantPreset::Standard))
        .expect("Failed to serialize VariantData");
}
