use crate::{
    Board, initializer,
    logic::{self},
    shared::{CastleSource, FactionId, TurnManager, VariantData, VariantPreset},
    tile::SpecialLayout,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub player_main_factions: Vec<FactionId>,
    pub turn_manager: TurnManager,
    pub remaining_castles: Vec<CastleSource>,
    pub variant_data: VariantData,
}

impl GameState {
    pub fn new(variant_data: VariantData) -> Option<Self> {
        let special_layout = SpecialLayout::new(variant_data.special_tile_pairs.clone())?;

        let board = Board::from_piece_list(
            &variant_data.initial_pieces,
            variant_data.board_width,
            variant_data.board_height,
            variant_data.promotion_area,
            special_layout,
        )?;

        let player_main_factions = variant_data.player_main_factions.clone();
        let turn_manager = TurnManager::new(variant_data.player_main_factions.len() as u32);
        let remaining_castles = logic::generate_castle(&board);
        Some(Self {
            board,
            player_main_factions,
            turn_manager,
            remaining_castles,
            variant_data,
        })
    }

    pub fn new_from_preset(variant_preset: VariantPreset) -> Self {
        let variant_data = match variant_preset {
            VariantPreset::Standard => initializer::variant::standard(),
            VariantPreset::Arena => initializer::variant::arena(),
        };
        Self::new(variant_data).expect(&format!("Preset {:?} is broken", variant_preset))
    }
}
