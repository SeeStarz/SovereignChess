use std::collections::HashMap;

use crate::{Area, Coordinate, FactionId, PieceSimple, faction, initializer, tile::SpecialLayout};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantData {
    pub board_width: u32,
    pub board_height: u32,
    pub promotion_area: Area,
    pub initial_pieces: HashMap<Coordinate, PieceSimple>,
    pub special_layout: SpecialLayout,
    pub player_main_factions: Vec<FactionId>,
    pub variant_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
    Standard,
    Arena,
}

impl From<Variant> for VariantData {
    fn from(value: Variant) -> Self {
        match value {
            Variant::Standard => VariantData {
                board_width: 16,
                board_height: 16,
                promotion_area: Area::new(6, 9, 6, 9),
                initial_pieces: initializer::board::standard(),
                special_layout: initializer::special::standard(),
                player_main_factions: vec![
                    FactionId::from(faction::White),
                    FactionId::from(faction::Black),
                ],
                variant_name: String::from("Standard"),
            },
            Variant::Arena => VariantData {
                board_width: 12,
                board_height: 12,
                promotion_area: Area::new(5, 6, 5, 6),
                initial_pieces: initializer::board::arena(),
                special_layout: initializer::special::arena(),
                player_main_factions: vec![
                    FactionId::from(faction::White),
                    FactionId::from(faction::Black),
                ],
                variant_name: String::from("Arena"),
            },
        }
    }
}
