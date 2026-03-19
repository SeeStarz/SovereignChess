use crate::engine::{
    Coordinate, Gamestate,
    faction::{self, Allegiance},
    legal_move::{
        LegalMove, NormalMove,
        calculate::{get_knight_directions, try_add_legal_move_check_special_tile_rules},
    },
};

pub fn add_knight_moves_naive(
    moves: &mut Vec<LegalMove>,
    gamestate: &Gamestate,
    faction: faction::Color,
    origin: Coordinate,
) {
    for &direction in get_knight_directions() {
        let Some(destination) = origin.offset(direction) else {
            continue;
        };
        if let Some(victim) = gamestate.c().board.at(destination) {
            if gamestate.get_allegiance(victim.faction) != Allegiance::Enemy {
                continue;
            }
        }

        try_add_legal_move_check_special_tile_rules(
            moves,
            gamestate,
            LegalMove::NormalMove(NormalMove {
                origin,
                destination,
            }),
            faction,
        );
    }
}
