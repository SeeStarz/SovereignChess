use super::super::{Coordinate, Piece, Tile};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    pub tiles: [[Tile; 16]; 16],
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [[None; 16]; 16],
        }
    }

    pub fn at(&self, coordinate: Coordinate) -> Tile {
        self.tiles[coordinate.row][coordinate.col]
    }

    pub fn set_at(&mut self, coordinate: Coordinate, piece: Option<Piece>) {
        self.tiles[coordinate.row][coordinate.col] = piece;
    }
}
