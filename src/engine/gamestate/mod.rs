mod board;
mod init_pieces;

use super::piece::Piece;
use board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gamestate {
    board: Board,
}

impl Gamestate {
    pub fn pieces(&self) -> Vec<Piece> {
        self.board
            .tiles
            .iter()
            .flatten()
            .filter_map(|&x| x)
            .collect()
    }

    pub fn new() -> Self {
        let mut board = Board::new();
        for (coordinate, piece) in init_pieces::normal() {
            assert!(board.at(coordinate).is_none());
            board.set_at(coordinate, Some(piece));
        }
        Self { board }
    }
}
