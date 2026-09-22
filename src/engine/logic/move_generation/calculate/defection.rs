use crate::engine::{
    GameState,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    {
        Coordinate, MoveRich,
        chess_move::DefectionRich,
        direction,
        faction::{self, Allegiance},
        tile,
    },
};

pub fn add_moves_naive(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    original_faction: faction::Color,
    origin: Coordinate,
) {
    let controlled_factions = game_state
        .derived
        .real_faction_owners
        .iter()
        .enumerate()
        .filter_map(|(faction, &owner)| {
            if owner.is_some_and(|owner| {
                owner == game_state.c().player_colors[game_state.c().turn_to_play as usize]
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
                if let Some(victim) = game_state.c().board.at(destination) {
                    if logic::get_allegiance(game_state, victim.faction) != Allegiance::Enemy {
                        continue;
                    }
                }

                try_add_move_check_special_tile_rules(
                    moves,
                    game_state,
                    MoveRich::Defection(DefectionRich {
                        faction: controlled_faction,
                        origin,
                        destination: Some(destination),
                    }),
                    controlled_faction,
                );
            }
        } else {
            moves.push(MoveRich::Defection(DefectionRich {
                faction: controlled_faction,
                origin,
                destination: None,
            }));
        }
    }
}
