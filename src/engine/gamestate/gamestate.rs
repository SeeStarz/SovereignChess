use crate::engine::{Coordinate, Move, board::Board, gamestate::init_pieces, piece::PieceExternal};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gamestate {
    board: Board,
}

impl Gamestate {
    pub fn pieces(&self) -> impl Iterator<Item = PieceExternal> {
        self.board
            .tiles
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter().enumerate().filter_map(move |(col, tile)| {
                    tile.map(|piece| {
                        PieceExternal::from_piece(
                            piece,
                            Coordinate::new_unchecked(row as i32, col as i32),
                        )
                    })
                })
            })
            .flatten()
    }

    pub fn new() -> Self {
        let mut board = Board::new();
        for (coordinate, piece) in init_pieces::normal() {
            assert!(board.at(coordinate).is_none());
            board.set_at(coordinate, Some(piece));
        }
        Self { board }
    }

    // TODO: Proper move logic
    pub fn apply_move(&self, move_: Move) -> Self {
        let mut board = self.board;
        if let Some(piece) = board.at(move_.origin) {
            board.set_at(move_.origin, None);
            board.set_at(move_.destination, Some(piece));
        }
        Self { board }
    }
}
