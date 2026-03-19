use crate::engine::{Coordinate, faction, piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegalMove {
    NormalMove(NormalMove),
    Castle(Castle),
    Promotion(Promotion),
    RegimeChangePromotion(RegimeChangePromotion),
    Defection(Defection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NormalMove {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Castle {
    pub king_move: NormalMove,
    pub rook_move: NormalMove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Promotion {
    pub normal_move: NormalMove,
    pub piece_type: piece::Type,
}

// Piece type is king
// Can be coup d'etat or overthrow, same thing here internally
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegimeChangePromotion {
    pub normal_move: NormalMove,
}

// Move can be None because moving only happens if defecting on a colored square the same color as chosen faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Defection {
    pub normal_move: Option<NormalMove>,
    pub faction: faction::Color,
}
