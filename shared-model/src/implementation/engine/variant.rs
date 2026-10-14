use crate::{
    definition::engine::{Area, FactionId, SeatId, VariantData, VariantPreset, faction},
    initializer,
};
use std::collections::HashMap;

impl From<VariantPreset> for VariantData {
    fn from(value: VariantPreset) -> Self {
        match value {
            VariantPreset::Standard => VariantData {
                board_width: 16,
                board_height: 16,
                promotion_area: Area::new(6, 9, 6, 9),
                initial_pieces: initializer::engine::piece::standard(),
                special_tile_pairs: initializer::engine::special::standard(),
                player_main_factions: HashMap::from([
                    (SeatId(0), FactionId::from(faction::White)),
                    (SeatId(1), FactionId::from(faction::Black)),
                ]),
                variant_name: String::from("Standard"),
                switch_prompt_enabled: true,
                strict_check: true,
            },
            VariantPreset::Arena => VariantData {
                board_width: 12,
                board_height: 12,
                promotion_area: Area::new(5, 6, 5, 6),
                initial_pieces: initializer::engine::piece::arena(),
                special_tile_pairs: initializer::engine::special::arena(),
                player_main_factions: HashMap::from([
                    (SeatId(0), FactionId::from(faction::White)),
                    (SeatId(1), FactionId::from(faction::Black)),
                ]),
                variant_name: String::from("Arena"),
                switch_prompt_enabled: true,
                strict_check: true,
            },
        }
    }
}
