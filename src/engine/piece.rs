use super::faction;

pub use piece::Type;
pub use piece::Type::*;
mod piece {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Type {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub faction: faction::Color,
    pub piece_type: piece::Type,
}
