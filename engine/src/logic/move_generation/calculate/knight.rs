use crate::{
    GameState, direction,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    shared::{Coordinate, FactionId, MoveRich, chess_move, faction},
};

pub fn add_moves_naive(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    faction: FactionId,
    origin: Coordinate,
) {
    for &direction in direction::knight() {
        let destination = origin.offset(direction);
        let Some(tile) = game_state.board.at(destination) else {
            continue;
        };
        if let Some(victim) = tile.piece {
            if logic::allegiance(game_state, victim.faction) != faction::Allegiance::Enemy {
                continue;
            }
        }

        try_add_move_check_special_tile_rules(
            moves,
            game_state,
            MoveRich::NormalMove(chess_move::Normal {
                origin,
                destination,
            }),
            faction,
        );
    }
}
