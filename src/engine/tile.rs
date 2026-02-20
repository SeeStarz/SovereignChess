use super::{coordinate::Coordinate, faction};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Special {
    pub faction: faction::Color,
    other: Coordinate,
}

impl Special {
    pub fn other(&self) -> &Special {
        SPECIAL_MAP
            .get(&self.other)
            .expect("Special tile has no pair")
    }

    pub fn at(coordinate: &Coordinate) -> Option<&Special> {
        SPECIAL_MAP.get(coordinate)
    }
}

lazy_static! {
    static ref SPECIAL_MAP: HashMap<Coordinate, Special> = HashMap::from([
        // White
        (Coordinate::new(7, 7), Special { faction: faction::White, other: Coordinate::new(8, 8) }),
        (Coordinate::new(8, 8), Special { faction: faction::White, other: Coordinate::new(7, 7) }),

        // Green
        (Coordinate::new(5, 10), Special { faction: faction::Green, other: Coordinate::new(10, 5) }),
        (Coordinate::new(10, 5), Special { faction: faction::Green, other: Coordinate::new(5, 10) }),

        // Ash
        (Coordinate::new(6, 9), Special { faction: faction::Ash, other: Coordinate::new(9, 6) }),
        (Coordinate::new(9, 6), Special { faction: faction::Ash, other: Coordinate::new(6, 9) }),

        // Cyan
        (Coordinate::new(7, 10), Special { faction: faction::Cyan, other: Coordinate::new(8, 5) }),
        (Coordinate::new(8, 5), Special { faction: faction::Cyan, other: Coordinate::new(7, 10) }),

        // Navy
        (Coordinate::new(4, 11), Special { faction: faction::Navy, other: Coordinate::new(11, 4) }),
        (Coordinate::new(11, 4), Special { faction: faction::Navy, other: Coordinate::new(4, 11) }),

        // Violet
        (Coordinate::new(5, 8), Special { faction: faction::Violet, other: Coordinate::new(10, 7) }),
        (Coordinate::new(10, 7), Special { faction: faction::Violet, other: Coordinate::new(5, 8) }),

        // Pink
        (Coordinate::new(5, 7), Special { faction: faction::Pink, other: Coordinate::new(10, 8) }),
        (Coordinate::new(10, 8), Special { faction: faction::Pink, other: Coordinate::new(5, 7) }),

        // Red
        (Coordinate::new(4, 4), Special { faction: faction::Red, other: Coordinate::new(11, 11) }),
        (Coordinate::new(11, 11), Special { faction: faction::Red, other: Coordinate::new(4, 4) }),

        // Orange
        (Coordinate::new(7, 5), Special { faction: faction::Orange, other: Coordinate::new(8, 10) }),
        (Coordinate::new(8, 10), Special { faction: faction::Orange, other: Coordinate::new(7, 5) }),

        // Slate
        (Coordinate::new(6, 6), Special { faction: faction::Slate, other: Coordinate::new(9, 9) }),
        (Coordinate::new(9, 9), Special { faction: faction::Slate, other: Coordinate::new(6, 6) }),

        // Yellow
        (Coordinate::new(5, 5), Special { faction: faction::Yellow, other: Coordinate::new(10, 10) }),
        (Coordinate::new(10, 10), Special { faction: faction::Yellow, other: Coordinate::new(5, 5) }),

        // Black
        (Coordinate::new(7, 8), Special { faction: faction::Black, other: Coordinate::new(8, 7) }),
        (Coordinate::new(8, 7), Special { faction: faction::Black, other: Coordinate::new(7, 8) }),
    ]);
}
