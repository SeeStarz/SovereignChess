use crate::definition::engine::{Area, Coordinate, FactionId, PieceSimple, tile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantData {
    pub board_width: u32,
    pub board_height: u32,
    pub promotion_area: Area,
    pub initial_pieces: Vec<(Coordinate, PieceSimple)>,
    pub special_tile_pairs: Vec<tile::SpecialPair>,
    pub player_main_factions: Vec<FactionId>,
    pub variant_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VariantPreset {
    Standard,
    Arena,
}
