use crate::engine::{
    Gamestate,
    gamestate::CanonicalState,
    model::{
        faction::{self, Allegiance, Color},
        tile,
    },
};
use strum::IntoEnumIterator;

pub fn get_real_faction_owners(state: &CanonicalState) -> [Option<faction::Color>; 12] {
    let direct_owners = {
        let mut direct_owners = [None; 12];
        for special in tile::Special::all() {
            let Some(piece) = state.board.at(special.coordinate) else {
                continue;
            };

            assert!(state.board.at(special.other().coordinate).is_none());
            direct_owners[special.faction as usize] = Some(piece.faction);
        }
        direct_owners
    };

    let mut real_owners = [None; 12];
    for faction in Color::iter() {
        let mut owner = faction;
        if state.player_colors.iter().any(|&f| f == owner) {
            real_owners[faction as usize] = Some(owner);
            continue;
        }

        while let Some(next) = direct_owners[owner as usize] {
            owner = next;

            if state.player_colors.iter().any(|&f| f == owner) {
                real_owners[faction as usize] = Some(owner);
                break;
            }
        }
    }
    real_owners
}

pub fn get_allegiance(gamestate: &Gamestate, faction: faction::Color) -> Allegiance {
    match gamestate.derived.real_faction_owners[faction as usize] {
        None => Allegiance::Neutral,
        Some(faction)
            if faction == gamestate.c().player_colors[gamestate.c().turn_to_play as usize] =>
        {
            Allegiance::Ally
        }
        Some(faction)
            if faction
                == gamestate.c().player_colors[gamestate.c().turn_to_play.other() as usize] =>
        {
            Allegiance::Enemy
        }
        _ => panic!("Faction {:?} is neither neutral, ally, nor enemy", faction),
    }
}
