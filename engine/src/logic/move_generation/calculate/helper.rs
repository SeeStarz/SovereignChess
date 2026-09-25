use crate::{FactionID, GameState, MoveRich, chess_move::NormalMove, logic};

/// # Panics
/// Panic if passed a castle move
pub fn try_add_move_check_special_tile_rules(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    chess_move: MoveRich,
    faction: FactionID,
) {
    let normal_move = match chess_move {
        MoveRich::NormalMove(normal_move) => normal_move,
        MoveRich::Promotion(promotion_move) => promotion_move.normal_move,
        MoveRich::RegimeChangePromotion(promotion_move) => promotion_move.pawn_move,
        MoveRich::Defection(defection_move) => {
            let origin = defection_move.origin;
            let destination = defection_move.destination.unwrap_or(origin);
            NormalMove {
                origin,
                destination,
            }
        }
        MoveRich::Castle(_castle_move) => panic!(),
    };

    let Some(special_destination) = game_state
        .board
        .special_layout()
        .at(normal_move.destination)
    else {
        moves.push(chess_move);
        return;
    };

    // Means that we are not trying to occupy special tile colored the same as current faction
    // We are also not trying to occupy special tile where there currently is a piece on the other pair
    if logic::is_special_tile_occupiable(&game_state.board, special_destination, faction) {
        moves.push(chess_move);
        return;
    }

    // If the current moved piece is the one on the other pair, it's safe to move there
    if let Some(special_origin) = game_state.board.special_layout().at(normal_move.origin)
        && game_state.board.special_layout().other(special_origin) == special_destination
    {
        moves.push(chess_move);
    }
}
