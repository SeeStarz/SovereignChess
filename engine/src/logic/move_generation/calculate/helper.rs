use crate::{
    GameState, logic,
    shared::{FactionId, MoveRich},
};

/// # Panics
/// Panic if passed a castle move
pub fn try_add_move_check_special_tile_rules(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    chess_move: MoveRich,
    faction: FactionId,
) {
    let (origin, destination) = match chess_move {
        MoveRich::NormalMove(normal_move) => (normal_move.origin, normal_move.destination),
        MoveRich::Promotion(promotion_move) => (
            promotion_move.normal_move.origin,
            promotion_move.normal_move.destination,
        ),
        MoveRich::RegimeChangePromotion(promotion_move) => (
            promotion_move.pawn_move.origin,
            promotion_move.pawn_move.destination,
        ),
        MoveRich::Defection(defection_move) => (
            defection_move.origin,
            defection_move.destination.unwrap_or(defection_move.origin),
        ),
        MoveRich::Castle(_castle_move) => {
            panic!("try_add_move_check_special_tile_rules can not handle check")
        }
    };

    if logic::is_possibly_special_tile_occupiable(&game_state.board, origin, destination, faction) {
        moves.push(chess_move);
        return;
    }
}
