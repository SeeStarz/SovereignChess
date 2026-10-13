use crate::{
    Board, TurnManager, initializer, logic,
    shared::{CastleSource, FactionId, MoveRich, MoveSimple, SeatId, VariantData, VariantPreset},
    tile::SpecialLayout,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub player_main_factions: HashMap<SeatId, FactionId>,
    pub turn_manager: TurnManager,
    pub remaining_castles: Vec<CastleSource>,
}

impl GameState {
    pub fn new(variant_data: &VariantData) -> Option<Self> {
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
        })
    }

    pub fn new_from_preset(variant_preset: VariantPreset) -> Self {
        let variant_data = match variant_preset {
            VariantPreset::Standard => initializer::variant::standard(),
            VariantPreset::Arena => initializer::variant::arena(),
        };

        Self::new(&variant_data).expect(&format!("Variant {:?} is broken", variant_preset))
    }

    pub fn apply_move_unchecked(&self, chess_move: MoveSimple) -> Self {
        logic::apply_move(self, chess_move)
    }

    pub fn apply_move(&self, chess_move: MoveSimple) -> Option<Self> {
        if self
            .legal_moves()
            .into_iter()
            .any(|m| MoveSimple::from(m) == chess_move)
        {
            Some(self.apply_move_unchecked(chess_move))
        } else {
            None
        }
    }

    pub fn legal_moves(&self) -> Vec<MoveRich> {
        logic::calculate_move(self)
    }
}
