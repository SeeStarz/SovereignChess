use crate::{
    Board, CastleSource, FactionId, MoveRich, MoveSimple, TurnManager, VariantData,
    logic::{self, board_pieces_rich},
    piece::PieceRich,
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
    pub fn new(variant_data: VariantData) -> Self {
        let board = Board::from_piece_hashmap(
            &variant_data.initial_pieces,
            variant_data.board_width,
            variant_data.board_height,
            variant_data.promotion_area,
            variant_data.special_layout.clone(),
        )
        .expect("Failed to initialize board");
        let player_main_factions = variant_data.player_main_factions.clone();
        let turn_manager = TurnManager::new(variant_data.player_main_factions.len() as u32);
        let remaining_castles = logic::generate_castle(&board);
        Self {
            board,
            player_main_factions,
            turn_manager,
            remaining_castles,
            variant_data,
        }
    }

    pub fn pieces(&self) -> impl Iterator<Item = PieceRich> {
        board_pieces_rich(self)
    }

    pub fn moves(&self) -> Vec<MoveRich> {
        logic::calculate_move(self)
    }

    pub fn apply_move(&self, chess_move: MoveSimple) -> Self {
        logic::apply_move(self, chess_move)
    }
}
