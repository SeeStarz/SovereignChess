use crate::definition::engine::{Coordinate, FactionId, piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSimpleRust {
    NormalMove(Normal),
    Castle(CastleSimple),
    Promotion(Promotion),
    RegimeChangePromotion(RegimeChangePromotionSimple),
    Defection(DefectionSimple),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveRichRust {
    NormalMove(Normal),
    Castle(CastleRich),
    Promotion(Promotion),
    RegimeChangePromotion(RegimeChangePromotionRich),
    Defection(DefectionRich),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveEnumC {
    NormalMove,
    Castle,
    Promotion,
    RegimeChangePromotion,
    Defection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Normal {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleSimple {
    pub king_destination: Coordinate,
    pub rook_move: Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleRich {
    pub king_move: Normal,
    pub rook_move: Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Promotion {
    pub normal_move: Normal,
    pub piece_type: piece::Type,
}

// Piece type is king
// Can be coup d'etat or overthrow, same thing here internally
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegimeChangePromotionSimple {
    pub pawn_move: Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegimeChangePromotionRich {
    pub pawn_move: Normal,
    pub king_origin: Option<Coordinate>,
}

// Move can be None because moving only happens if defecting on a colored square the same color as chosen faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefectionSimple {
    pub destination: Option<Coordinate>,
    pub faction: FactionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefectionRich {
    pub origin: Coordinate,
    pub destination: Option<Coordinate>,
    pub faction: FactionId,
}
