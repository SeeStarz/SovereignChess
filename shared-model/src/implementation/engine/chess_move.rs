use crate::definition::engine::chess_move::{
    CastleRich, CastleSimple, DefectionRich, DefectionSimple, MoveRich, MoveSimple,
    RegimeChangePromotionRich, RegimeChangePromotionSimple,
};

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

impl From<CastleRich> for CastleSimple {
    fn from(castle: CastleRich) -> Self {
        CastleSimple {
            king_destination: castle.king_move.destination,
            rook_move: castle.rook_move,
        }
    }
}

impl From<RegimeChangePromotionRich> for RegimeChangePromotionSimple {
    fn from(promotion: RegimeChangePromotionRich) -> Self {
        RegimeChangePromotionSimple {
            pawn_move: promotion.pawn_move,
        }
    }
}

impl From<DefectionRich> for DefectionSimple {
    fn from(defection: DefectionRich) -> Self {
        DefectionSimple {
            destination: defection.destination,
            faction: defection.faction,
        }
    }
}
