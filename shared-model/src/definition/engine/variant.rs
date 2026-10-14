use crate::definition::engine::{Area, Coordinate, FactionId, PieceSimple, SeatId, tile};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VariantData {
    pub board_width: u32,
    pub board_height: u32,
    pub promotion_area: Area,
    #[cfg_attr(feature = "serde", serde(with = "crate::helper::hashmap_as_vec"))]
    pub initial_pieces: HashMap<Coordinate, PieceSimple>,
    pub special_tile_pairs: Vec<tile::SpecialPair>,
    pub player_main_factions: HashMap<SeatId, FactionId>,
    pub variant_name: String,
    pub switch_prompt_enabled: bool,
    pub strict_check: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VariantPreset {
    Standard,
    Arena,
}
