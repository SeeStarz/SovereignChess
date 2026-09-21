use crate::engine::{
    GameState, logic,
    logic::move_generation::calculate::helper::try_add_move_check_special_tile_rules,
    model::{
        Coordinate,
        chess_move::{Move, NormalMove},
        direction,
        faction::{self, Allegiance},
    },
};

pub fn add_moves_naive(
    moves: &mut Vec<Move>,
    game_state: &GameState,
    faction: faction::Color,
    origin: Coordinate,
) {
    for &direction in direction::knight() {
        let Some(destination) = origin.offset(direction) else {
            continue;
        };
        if let Some(victim) = game_state.c().board.at(destination) {
            if logic::faction::get_allegiance(game_state, victim.faction) != Allegiance::Enemy {
                continue;
            }
        }

        try_add_move_check_special_tile_rules(
            moves,
            game_state,
            Move::NormalMove(NormalMove {
                origin,
                destination,
            }),
            faction,
        );
    }
}
