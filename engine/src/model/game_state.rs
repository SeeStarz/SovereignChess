use crate::{
    Area, Board, CastleSource, FactionID, MoveRich, MoveSimple, TurnManager, faction, initializer,
    logic::{self, board_pieces_rich},
    piece::PieceRich,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    pub board: Board,
    pub player_colors: Vec<FactionID>,
    pub turn_manager: TurnManager,
    pub remaining_castles: Vec<CastleSource>,
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::from_piece_hashmap(
            &initializer::board::normal::generate(),
            16,
            16,
            Area::new(6, 9, 6, 9),
            initializer::special::standard_layout(),
        )
        .expect("Failed to initialize board");
        let player_colors = vec![
            FactionID::from(faction::White),
            FactionID::from(faction::Black),
        ];
        let turn_manager = TurnManager::new(2);
        let remaining_castles = logic::generate_castle(&board);
        Self {
            board,
            player_colors,
            turn_manager,
            remaining_castles,
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
