use crate::{
    Coordinate, FactionID, GameState, MoveRich,
    chess_move::DefectionRich,
    direction,
    faction::Allegiance,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    tile,
};

pub fn add_moves_naive(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    original_faction: FactionID,
    origin: Coordinate,
) {
    let real_faction_owners = logic::real_faction_owners(game_state);
    let controlled_factions = real_faction_owners.iter().filter_map(|(&faction, &owner)| {
        if owner == logic::current_player_faction(game_state) && faction != original_faction {
            Some(faction)
        } else {
            None
        }
    });

    for controlled_faction in controlled_factions {
        if tile::Special::at(origin).is_some_and(|&t| t.faction == controlled_faction) {
            for &direction in direction::queen() {
                let destination = origin.offset(direction);
                let Some(tile) = game_state.board.at(destination) else {
                    continue;
                };
                if let Some(victim) = tile.0 {
                    if logic::allegiance(game_state, victim.faction) != Allegiance::Enemy {
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
