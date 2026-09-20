use crate::engine::{
    Gamestate,
    logic::{
        self, move_generation::calculate::helper::try_add_legal_move_check_special_tile_rules,
    },
    model::{
        Coordinate,
        chess_move::{Defection, Move, NormalMove},
        direction,
        faction::{self, Allegiance},
        tile,
    },
};

pub fn add_moves_naive(
    moves: &mut Vec<Move>,
    gamestate: &Gamestate,
    original_faction: faction::Color,
    origin: Coordinate,
) {
    let controlled_factions = gamestate
        .derived
        .real_faction_owners
        .iter()
        .enumerate()
        .filter_map(|(faction, &owner)| {
            if owner.is_some_and(|owner| {
                owner == gamestate.c().player_colors[gamestate.c().turn_to_play as usize]
                    && faction != original_faction as usize
            }) {
                Some(
                    faction::Color::from_repr(faction)
                        .expect("Internal logic error invalid enum repr"),
                )
            } else {
                None
            }
        });

    for controlled_faction in controlled_factions {
        if tile::Special::at(origin).is_some_and(|&t| t.faction == controlled_faction) {
            for &direction in direction::queen() {
                let Some(destination) = origin.offset(direction) else {
                    continue;
                };
                if let Some(victim) = gamestate.c().board.at(destination) {
                    if logic::faction::get_allegiance(gamestate, victim.faction)
                        != Allegiance::Enemy
                    {
                        continue;
                    }
                }

                try_add_legal_move_check_special_tile_rules(
                    moves,
                    gamestate,
                    Move::Defection(Defection {
                        faction: controlled_faction,
                        normal_move: Some(NormalMove {
                            origin,
                            destination,
                        }),
                    }),
                    controlled_faction,
                );
            }
        } else {
            moves.push(Move::Defection(Defection {
                faction: controlled_faction,
                normal_move: None,
            }));
        }
    }
}
