use crate::{Coordinate, FactionID, PieceRich, PieceSimple, PieceWithCoordinate, faction};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileSimple(pub Option<PieceSimple>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileWithCoordinate {
    pub piece: Option<PieceWithCoordinate>,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileRich {
    pub piece: Option<PieceRich>,
    pub special: Option<Special>,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Special {
    pub faction: FactionID,
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

    pub fn with_color(faction: FactionID) -> &'static [Special] {
        SPECIAL_MAP_BY_FACTION.get(&faction).expect(&format!(
            "Faction {:?} does not have special tiles",
            faction
        ))
    }

    pub fn all() -> &'static [Special] {
        &*SPECIAL_TILES_DEFAULT
    }
}

lazy_static! {
    static ref SPECIAL_MAP_BY_COORDINATE: HashMap<Coordinate, Special> = SPECIAL_TILES_DEFAULT
        .iter()
        .map(|&t| (t.coordinate, t))
        .collect();
    static ref SPECIAL_MAP_BY_FACTION: HashMap<FactionID, [Special; 2]> = {
        let mut map = HashMap::new();
        for i in (0..24).step_by(2) {
            map.insert(
                SPECIAL_TILES_DEFAULT[i].faction,
                [SPECIAL_TILES_DEFAULT[i], SPECIAL_TILES_DEFAULT[i + 1]],
            );
            assert!(SPECIAL_TILES_DEFAULT[i].faction == SPECIAL_TILES_DEFAULT[i + 1].faction)
        }
        map
    };

    // ORDER IS IMPORTANT
    static ref SPECIAL_TILES_DEFAULT: [Special; 24] = [
        // White
        Special {
            faction: FactionID::from(faction::White),
            coordinate: Coordinate::new(7, 7),
            other_coordinate: Coordinate::new(8, 8),
        },
        Special {
            faction: FactionID::from(faction::White),
            coordinate: Coordinate::new(8, 8),
            other_coordinate: Coordinate::new(7, 7),
        },
        // Green
        Special {
            faction: FactionID::from(faction::Green),
            coordinate: Coordinate::new(5, 10),
            other_coordinate: Coordinate::new(10, 5),
        },
        Special {
            faction: FactionID::from(faction::Green),
            coordinate: Coordinate::new(10, 5),
            other_coordinate: Coordinate::new(5, 10),
        },
        // Ash
        Special {
            faction: FactionID::from(faction::Ash),
            coordinate: Coordinate::new(6, 9),
            other_coordinate: Coordinate::new(9, 6),
        },
        Special {
            faction: FactionID::from(faction::Ash),
            coordinate: Coordinate::new(9, 6),
            other_coordinate: Coordinate::new(6, 9),
        },
        // Cyan
        Special {
            faction: FactionID::from(faction::Cyan),
            coordinate: Coordinate::new(7, 10),
            other_coordinate: Coordinate::new(8, 5),
        },
        Special {
            faction: FactionID::from(faction::Cyan),
            coordinate: Coordinate::new(8, 5),
            other_coordinate: Coordinate::new(7, 10),
        },
        // Navy
        Special {
            faction: FactionID::from(faction::Navy),
            coordinate: Coordinate::new(4, 11),
            other_coordinate: Coordinate::new(11, 4),
        },
        Special {
            faction: FactionID::from(faction::Navy),
            coordinate: Coordinate::new(11, 4),
            other_coordinate: Coordinate::new(4, 11),
        },
        // Violet
        Special {
            faction: FactionID::from(faction::Violet),
            coordinate: Coordinate::new(5, 8),
            other_coordinate: Coordinate::new(10, 7),
        },
        Special {
            faction: FactionID::from(faction::Violet),
            coordinate: Coordinate::new(10, 7),
            other_coordinate: Coordinate::new(5, 8),
        },
        // Pink
        Special {
            faction: FactionID::from(faction::Pink),
            coordinate: Coordinate::new(5, 7),
            other_coordinate: Coordinate::new(10, 8),
        },
        Special {
            faction: FactionID::from(faction::Pink),
            coordinate: Coordinate::new(10, 8),
            other_coordinate: Coordinate::new(5, 7),
        },
        // Red
        Special {
            faction: FactionID::from(faction::Red),
            coordinate: Coordinate::new(4, 4),
            other_coordinate: Coordinate::new(11, 11),
        },
        Special {
            faction: FactionID::from(faction::Red),
            coordinate: Coordinate::new(11, 11),
            other_coordinate: Coordinate::new(4, 4),
        },
        // Orange
        Special {
            faction: FactionID::from(faction::Orange),
            coordinate: Coordinate::new(7, 5),
            other_coordinate: Coordinate::new(8, 10),
        },
        Special {
            faction: FactionID::from(faction::Orange),
            coordinate: Coordinate::new(8, 10),
            other_coordinate: Coordinate::new(7, 5),
        },
        // Slate
        Special {
            faction: FactionID::from(faction::Slate),
            coordinate: Coordinate::new(6, 6),
            other_coordinate: Coordinate::new(9, 9),
        },
        Special {
            faction: FactionID::from(faction::Slate),
            coordinate: Coordinate::new(9, 9),
            other_coordinate: Coordinate::new(6, 6),
        },
        // Yellow
        Special {
            faction: FactionID::from(faction::Yellow),
            coordinate: Coordinate::new(5, 5),
            other_coordinate: Coordinate::new(10, 10),
        },
        Special {
            faction: FactionID::from(faction::Yellow),
            coordinate: Coordinate::new(10, 10),
            other_coordinate: Coordinate::new(5, 5),
        },
        // Black
        Special {
            faction: FactionID::from(faction::Black),
            coordinate: Coordinate::new(7, 8),
            other_coordinate: Coordinate::new(8, 7),
        },
        Special {
            faction: FactionID::from(faction::Black),
            coordinate: Coordinate::new(8, 7),
            other_coordinate: Coordinate::new(7, 8),
        },
    ];
}
