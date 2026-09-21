use crate::engine::{
    GameState, logic,
    model::{Move, faction, tile},
};

pub fn try_add_move_check_special_tile_rules(
    moves: &mut Vec<Move>,
    game_state: &GameState,
    chess_move: Move,
    faction: faction::Color,
) {
    let normal_move = match chess_move {
        Move::NormalMove(normal_move) => normal_move,
        Move::Promotion(promotion_move) => promotion_move.normal_move,
        Move::RegimeChangePromotion(promotion_move) => promotion_move.normal_move,
        Move::Defection(defection_move) => defection_move.normal_move.unwrap(),
        _ => panic!(),
    };

    let Some(&special_destination) = tile::Special::at(normal_move.destination) else {
        moves.push(chess_move);
        return;
    };

    // Means that we are not trying to occupy special tile colored the same as current faction
    // We are also not trying to occupy special tile where there currently is a piece on the other pair
    if logic::special::is_special_tile_occupiable(
        &game_state.c().board,
        special_destination,
        faction,
    ) {
        moves.push(chess_move);
        return;
    }

    // If the current moved piece is the one on the other pair, it's safe to move there
    if let Some(&special_origin) = tile::Special::at(normal_move.origin)
        && special_origin.other() == special_destination
    {
        moves.push(chess_move);
    }
}
