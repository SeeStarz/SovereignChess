use crate::definition::engine::chess_move::{
    CastleRich, CastleSimple, DefectionRich, DefectionSimple, MoveRichRust, MoveSimpleRust,
    RegimeChangePromotionRich, RegimeChangePromotionSimple,
};

impl From<MoveRichRust> for MoveSimpleRust {
    fn from(chess_move: MoveRichRust) -> Self {
        match chess_move {
            MoveRichRust::Castle(castle_move) => {
                MoveSimpleRust::Castle(CastleSimple::from(castle_move))
            }
            MoveRichRust::Defection(defection_move) => {
                MoveSimpleRust::Defection(DefectionSimple::from(defection_move))
            }
            MoveRichRust::RegimeChangePromotion(promotion_move) => {
                MoveSimpleRust::RegimeChangePromotion(RegimeChangePromotionSimple::from(
                    promotion_move,
                ))
            }
            MoveRichRust::Promotion(promotion_move) => MoveSimpleRust::Promotion(promotion_move),
            MoveRichRust::NormalMove(normal_move) => MoveSimpleRust::NormalMove(normal_move),
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
