use crate::{
    Board, GameState,
    shared::{FactionId, faction::Allegiance},
};
use std::collections::{HashMap, HashSet};

pub fn real_faction_owners(game_state: &GameState) -> HashMap<FactionId, FactionId> {
    let direct_owners = {
        let mut direct_owners: HashMap<FactionId, FactionId> = HashMap::new();

        for special_tile_pair in game_state.board.special_layout().all() {
            let Some(piece) = game_state
                .board
                .at(special_tile_pair.coordinates[0])
                .expect("Special tile out of bounds")
                .piece
                .or(game_state
                    .board
                    .at(special_tile_pair.coordinates[1])
                    .expect("Special tile out of bounds")
                    .piece)
            else {
                continue;
            };

            direct_owners.insert(special_tile_pair.faction, piece.faction);
        }
        direct_owners
    };

    let mut real_owners: HashMap<FactionId, FactionId> = HashMap::new();
    for (&faction, &owner) in direct_owners.iter() {
        let mut owner = owner;
        while let Some(&next) = direct_owners.get(&owner) {
            owner = next;

            if game_state.player_main_factions.iter().any(|&f| f == owner) {
                real_owners.insert(faction, owner);
                break;
            }
        }
    }
    for &main_faction in game_state.player_main_factions.iter() {
        real_owners.insert(main_faction, main_faction);
    }
    real_owners
}

pub fn all(board: &Board) -> impl Iterator<Item = FactionId> {
    let hash_set: HashSet<FactionId> = board
        .special_layout()
        .all()
        .into_iter()
        .map(|s| s.faction)
        .collect();
    hash_set.into_iter()
}

pub fn allegiance(game_state: &GameState, faction: FactionId) -> Allegiance {
    let real_faction_owners = real_faction_owners(game_state);
    match real_faction_owners.get(&faction).cloned() {
        None => Allegiance::Neutral,
        Some(faction) => {
            if faction == current_player_faction(game_state) {
                Allegiance::Ally
            } else {
                Allegiance::Enemy
            }
        }
    }
}

pub fn current_player_faction(game_state: &GameState) -> FactionId {
    game_state.player_main_factions[game_state.turn_manager.current_player().0 as usize]
}

pub fn is_faction_in_board(board: &Board, faction: FactionId) -> bool {
    all(board).any(|f| f == faction)
}
