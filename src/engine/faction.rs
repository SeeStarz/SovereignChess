use crate::engine::{Gamestate, faction, tile};
use strum::{EnumIter, IntoEnumIterator};

pub use Color::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Color {
    White,
    Pink,
    Slate,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Navy,
    Ash,
    Violet,
    Black,
}

impl Gamestate {
    pub(in crate::engine) fn get_faction_owners(&self) -> [Option<faction::Color>; 12] {
        let direct_owners = {
            let mut direct_owners = [None; 12];
            for special in tile::Special::all() {
                let Some(piece) = self.c().board.at(special.coordinate) else {
                    continue;
                };

                assert!(self.c().board.at(special.other().coordinate).is_none());
                direct_owners[special.faction as usize] = Some(piece.faction);
            }
            direct_owners
        };

        let mut real_owners = [None; 12];
        for faction in Color::iter() {
            let mut owner = faction;
            while let Some(next) = direct_owners[owner as usize] {
                owner = next;
            }

            if self.c().player_colors.iter().any(|&f| f == owner) {
                real_owners[faction as usize] = Some(owner);
            }
        }
        real_owners
    }
}
