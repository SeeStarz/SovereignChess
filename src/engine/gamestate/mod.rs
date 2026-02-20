mod init_pieces;

use super::{Coordinate, piece::Piece};
use std::collections::HashMap;

pub struct Gamestate {
    pieces: HashMap<Coordinate, Piece>,
}

impl Gamestate {
    pub fn pieces(&self) -> Vec<&Piece> {
        self.pieces.values().collect()
    }

    pub fn new() -> Self {
        Self {
            pieces: init_pieces::normal(),
        }
    }
}
