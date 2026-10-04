use crate::{
    initializer,
    shared::{Area, FactionId, VariantData, faction},
};

pub fn standard() -> VariantData {
    VariantData {
        board_width: 16,
        board_height: 16,
        promotion_area: Area::new(6, 9, 6, 9),
        initial_pieces: initializer::board::standard(),
        special_tile_pairs: initializer::special::standard(),
        player_main_factions: vec![
            FactionId::from(faction::White),
            FactionId::from(faction::Black),
        ],
        variant_name: String::from("Standard"),
    }
}

pub fn arena() -> VariantData {
    VariantData {
        board_width: 12,
        board_height: 12,
        promotion_area: Area::new(5, 6, 5, 6),
        initial_pieces: initializer::board::arena(),
        special_tile_pairs: initializer::special::arena(),
        player_main_factions: vec![
            FactionId::from(faction::White),
            FactionId::from(faction::Black),
        ],
        variant_name: String::from("Arena"),
    }
}
