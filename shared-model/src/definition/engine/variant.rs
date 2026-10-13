use std::collections::HashMap;

use crate::definition::engine::{Area, Coordinate, FactionId, PieceSimple, SeatId, tile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantData {
    pub board_width: u32,
    pub board_height: u32,
    pub promotion_area: Area,
    pub initial_pieces: HashMap<Coordinate, PieceSimple>,
    pub special_tile_pairs: Vec<tile::SpecialPair>,
    pub player_main_factions: HashMap<SeatId, FactionId>,
    pub variant_name: String,
    pub switch_prompt_enabled: bool,
    pub strict_check: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariantPreset {
    Standard,
    Arena,
}
