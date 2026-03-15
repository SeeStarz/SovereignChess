use crate::engine::{Gamestate, Piece, coordinate::Coordinate, faction};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Tile = Option<Piece>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Special {
    pub faction: faction::Color,
    pub coordinate: Coordinate,
    other_coordinate: Coordinate,
}

impl Special {
    pub fn other(&self) -> Special {
        *SPECIAL_MAP_BY_COORDINATE
            .get(&self.other_coordinate)
            .expect("Special tile has no pair")
    }

    pub fn at(coordinate: Coordinate) -> Option<&'static Special> {
        SPECIAL_MAP_BY_COORDINATE.get(&coordinate).to_owned()
    }

    pub fn with_color(faction: faction::Color) -> &'static [Special] {
        SPECIAL_MAP_BY_FACTION.get(&faction).expect(&format!(
            "Faction {:?} does not have special tiles",
            faction
        ))
    }

    pub fn all() -> &'static [Special] {
        &SPECIAL_TILES
    }
}

impl Gamestate {
    pub(in crate::engine) fn is_special_tile_occupiable(
        &self,
        special: Special,
        faction: faction::Color,
    ) -> bool {
        if special.faction == faction {
            return false;
        }

        if let Some(_) = self.c().board.at(special.other().coordinate) {
            false
        } else {
            true
        }
    }
}

lazy_static! {
    static ref SPECIAL_MAP_BY_COORDINATE: HashMap<Coordinate, Special> =
        SPECIAL_TILES.iter().map(|&t| (t.coordinate, t)).collect();
    static ref SPECIAL_MAP_BY_FACTION: HashMap<faction::Color, [Special; 2]> = {
        let mut map = HashMap::new();
        for i in (0..24).step_by(2) {
            map.insert(
                SPECIAL_TILES[i].faction,
                [SPECIAL_TILES[i], SPECIAL_TILES[i + 1]],
            );
            assert!(SPECIAL_TILES[i].faction == SPECIAL_TILES[i + 1].faction)
        }
        map
    };
}

// ORDER IS IMPORTANT
static SPECIAL_TILES: [Special; 24] = [
    // White
    Special {
        faction: faction::White,
        coordinate: Coordinate::new_unchecked(7, 7),
        other_coordinate: Coordinate::new_unchecked(8, 8),
    },
    Special {
        faction: faction::White,
        coordinate: Coordinate::new_unchecked(8, 8),
        other_coordinate: Coordinate::new_unchecked(7, 7),
    },
    // Green
    Special {
        faction: faction::Green,
        coordinate: Coordinate::new_unchecked(5, 10),
        other_coordinate: Coordinate::new_unchecked(10, 5),
    },
    Special {
        faction: faction::Green,
        coordinate: Coordinate::new_unchecked(10, 5),
        other_coordinate: Coordinate::new_unchecked(5, 10),
    },
    // Ash
    Special {
        faction: faction::Ash,
        coordinate: Coordinate::new_unchecked(6, 9),
        other_coordinate: Coordinate::new_unchecked(9, 6),
    },
    Special {
        faction: faction::Ash,
        coordinate: Coordinate::new_unchecked(9, 6),
        other_coordinate: Coordinate::new_unchecked(6, 9),
    },
    // Cyan
    Special {
        faction: faction::Cyan,
        coordinate: Coordinate::new_unchecked(7, 10),
        other_coordinate: Coordinate::new_unchecked(8, 5),
    },
    Special {
        faction: faction::Cyan,
        coordinate: Coordinate::new_unchecked(8, 5),
        other_coordinate: Coordinate::new_unchecked(7, 10),
    },
    // Navy
    Special {
        faction: faction::Navy,
        coordinate: Coordinate::new_unchecked(4, 11),
        other_coordinate: Coordinate::new_unchecked(11, 4),
    },
    Special {
        faction: faction::Navy,
        coordinate: Coordinate::new_unchecked(11, 4),
        other_coordinate: Coordinate::new_unchecked(4, 11),
    },
    // Violet
    Special {
        faction: faction::Violet,
        coordinate: Coordinate::new_unchecked(5, 8),
        other_coordinate: Coordinate::new_unchecked(10, 7),
    },
    Special {
        faction: faction::Violet,
        coordinate: Coordinate::new_unchecked(10, 7),
        other_coordinate: Coordinate::new_unchecked(5, 8),
    },
    // Pink
    Special {
        faction: faction::Pink,
        coordinate: Coordinate::new_unchecked(5, 7),
        other_coordinate: Coordinate::new_unchecked(10, 8),
    },
    Special {
        faction: faction::Pink,
        coordinate: Coordinate::new_unchecked(10, 8),
        other_coordinate: Coordinate::new_unchecked(5, 7),
    },
    // Red
    Special {
        faction: faction::Red,
        coordinate: Coordinate::new_unchecked(4, 4),
        other_coordinate: Coordinate::new_unchecked(11, 11),
    },
    Special {
        faction: faction::Red,
        coordinate: Coordinate::new_unchecked(11, 11),
        other_coordinate: Coordinate::new_unchecked(4, 4),
    },
    // Orange
    Special {
        faction: faction::Orange,
        coordinate: Coordinate::new_unchecked(7, 5),
        other_coordinate: Coordinate::new_unchecked(8, 10),
    },
    Special {
        faction: faction::Orange,
        coordinate: Coordinate::new_unchecked(8, 10),
        other_coordinate: Coordinate::new_unchecked(7, 5),
    },
    // Slate
    Special {
        faction: faction::Slate,
        coordinate: Coordinate::new_unchecked(6, 6),
        other_coordinate: Coordinate::new_unchecked(9, 9),
    },
    Special {
        faction: faction::Slate,
        coordinate: Coordinate::new_unchecked(9, 9),
        other_coordinate: Coordinate::new_unchecked(6, 6),
    },
    // Yellow
    Special {
        faction: faction::Yellow,
        coordinate: Coordinate::new_unchecked(5, 5),
        other_coordinate: Coordinate::new_unchecked(10, 10),
    },
    Special {
        faction: faction::Yellow,
        coordinate: Coordinate::new_unchecked(10, 10),
        other_coordinate: Coordinate::new_unchecked(5, 5),
    },
    // Black
    Special {
        faction: faction::Black,
        coordinate: Coordinate::new_unchecked(7, 8),
        other_coordinate: Coordinate::new_unchecked(8, 7),
    },
    Special {
        faction: faction::Black,
        coordinate: Coordinate::new_unchecked(8, 7),
        other_coordinate: Coordinate::new_unchecked(7, 8),
    },
];
