use crate::engine::{
    initializer,
    model::{Coordinate, Piece, Tile},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    pub tiles: [[Tile; 16]; 16],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            tiles: [[None; 16]; 16],
        }
    }

    pub fn default() -> Board {
        let mut board = Self::empty();
        initializer::board::normal::generate()
            .into_iter()
            .for_each(|(coordinate, piece)| board.set_at(coordinate, Some(piece)));
        board
    }

    pub fn at(&self, coordinate: Coordinate) -> Tile {
        self.tiles[coordinate.row()][coordinate.col()]
    }

    pub fn set_at(&mut self, coordinate: Coordinate, piece: Option<Piece>) {
        self.tiles[coordinate.row()][coordinate.col()] = piece;
    }
}
