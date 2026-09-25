use crate::{Coordinate, FactionId, piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSimple {
    NormalMove(NormalMove),
    Castle(CastleSimple),
    Promotion(Promotion),
    RegimeChangePromotion(RegimeChangePromotionSimple),
    Defection(DefectionSimple),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveRich {
    NormalMove(NormalMove),
    Castle(CastleRich),
    Promotion(Promotion),
    RegimeChangePromotion(RegimeChangePromotionRich),
    Defection(DefectionRich),
}

impl From<MoveRich> for MoveSimple {
    fn from(chess_move: MoveRich) -> Self {
        match chess_move {
            MoveRich::Castle(castle_move) => MoveSimple::Castle(CastleSimple::from(castle_move)),
            MoveRich::Defection(defection_move) => {
                MoveSimple::Defection(DefectionSimple::from(defection_move))
            }
            MoveRich::RegimeChangePromotion(promotion_move) => {
                MoveSimple::RegimeChangePromotion(RegimeChangePromotionSimple::from(promotion_move))
            }
            MoveRich::Promotion(promotion_move) => MoveSimple::Promotion(promotion_move),
            MoveRich::NormalMove(normal_move) => MoveSimple::NormalMove(normal_move),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NormalMove {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleSimple {
    pub king_destination: Coordinate,
    pub rook_move: NormalMove,
}

impl From<CastleRich> for CastleSimple {
    fn from(castle: CastleRich) -> Self {
        CastleSimple {
            king_destination: castle.king_move.destination,
            rook_move: castle.rook_move,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleRich {
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
pub struct RegimeChangePromotionSimple {
    pub pawn_move: NormalMove,
}

impl From<RegimeChangePromotionRich> for RegimeChangePromotionSimple {
    fn from(promotion: RegimeChangePromotionRich) -> Self {
        RegimeChangePromotionSimple {
            pawn_move: promotion.pawn_move,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegimeChangePromotionRich {
    pub pawn_move: NormalMove,
    pub king_origin: Option<Coordinate>,
}

// Move can be None because moving only happens if defecting on a colored square the same color as chosen faction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefectionSimple {
    pub destination: Option<Coordinate>,
    pub faction: FactionId,
}

impl From<DefectionRich> for DefectionSimple {
    fn from(defection: DefectionRich) -> Self {
        DefectionSimple {
            destination: defection.destination,
            faction: defection.faction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefectionRich {
    pub origin: Coordinate,
    pub destination: Option<Coordinate>,
    pub faction: FactionId,
}
