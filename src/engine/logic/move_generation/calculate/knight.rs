use crate::engine::{
    Gamestate, logic,
    logic::move_generation::calculate::helper::try_add_legal_move_check_special_tile_rules,
    model::{
        Coordinate,
        chess_move::{Move, NormalMove},
        direction,
        faction::{self, Allegiance},
    },
};

pub fn add_moves_naive(
    moves: &mut Vec<Move>,
    gamestate: &Gamestate,
    faction: faction::Color,
    origin: Coordinate,
) {
    for &direction in direction::knight() {
        let Some(destination) = origin.offset(direction) else {
            continue;
        };
        if let Some(victim) = gamestate.c().board.at(destination) {
            if logic::faction::get_allegiance(gamestate, victim.faction) != Allegiance::Enemy {
                continue;
            }
        }

        try_add_legal_move_check_special_tile_rules(
            moves,
            gamestate,
            Move::NormalMove(NormalMove {
                origin,
                destination,
            }),
            faction,
        );
    }
}
