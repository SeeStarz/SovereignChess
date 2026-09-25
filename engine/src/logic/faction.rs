use crate::{
    FactionID, GameState,
    faction::{self, Allegiance},
    tile,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub fn real_faction_owners(game_state: &GameState) -> HashMap<FactionID, FactionID> {
    let direct_owners = {
        let mut direct_owners: HashMap<FactionID, FactionID> = HashMap::new();

        for special in tile::Special::all() {
            let Some(piece) = game_state
                .board
                .at(special.coordinate)
                .expect("Special tile out of bounds")
                .0
            else {
                continue;
            };

            assert!(
                game_state
                    .board
                    .at(special.other().coordinate)
                    .expect("Special tile out of bounds")
                    .0
                    .is_none()
            );
            direct_owners.insert(special.faction, piece.faction);
        }
        direct_owners
    };

    let mut real_owners: HashMap<FactionID, FactionID> = HashMap::new();
    for faction in faction::ColorDefault::iter().map(|c| FactionID::from(c)) {
        let mut owner = faction;
        if game_state.player_colors.iter().any(|&f| f == owner) {
            real_owners.insert(faction, owner);
            continue;
        }

        while let Some(&next) = direct_owners.get(&owner) {
            owner = next;

            if game_state.player_colors.iter().any(|&f| f == owner) {
                real_owners.insert(faction, owner);
                break;
            }
        }
    }
    real_owners
}

pub fn allegiance(game_state: &GameState, faction: FactionID) -> Allegiance {
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

pub fn current_player_faction(game_state: &GameState) -> FactionID {
    game_state.player_colors[game_state.turn_manager.current_player().0 as usize]
}
