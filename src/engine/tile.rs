use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::engine::coordinate::Coordinate;

use super::faction;

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
        (Coordinate::new(7, 7), Special { faction: faction::Color::White, other: Coordinate::new(8, 8) }),
        (Coordinate::new(8, 8), Special { faction: faction::Color::White, other: Coordinate::new(7, 7) }),

        // Green
        (Coordinate::new(5, 10), Special { faction: faction::Color::Green, other: Coordinate::new(10, 5) }),
        (Coordinate::new(10, 5), Special { faction: faction::Color::Green, other: Coordinate::new(5, 10) }),

        // Ash
        (Coordinate::new(6, 9), Special { faction: faction::Color::Ash, other: Coordinate::new(9, 6) }),
        (Coordinate::new(9, 6), Special { faction: faction::Color::Ash, other: Coordinate::new(6, 9) }),

        // Cyan
        (Coordinate::new(7, 10), Special { faction: faction::Color::Cyan, other: Coordinate::new(8, 5) }),
        (Coordinate::new(8, 5), Special { faction: faction::Color::Cyan, other: Coordinate::new(7, 10) }),

        // Navy
        (Coordinate::new(4, 11), Special { faction: faction::Color::Navy, other: Coordinate::new(11, 4) }),
        (Coordinate::new(11, 4), Special { faction: faction::Color::Navy, other: Coordinate::new(4, 11) }),

        // Violet
        (Coordinate::new(5, 8), Special { faction: faction::Color::Violet, other: Coordinate::new(10, 7) }),
        (Coordinate::new(10, 7), Special { faction: faction::Color::Violet, other: Coordinate::new(5, 8) }),

        // Pink
        (Coordinate::new(5, 7), Special { faction: faction::Color::Pink, other: Coordinate::new(10, 8) }),
        (Coordinate::new(10, 8), Special { faction: faction::Color::Pink, other: Coordinate::new(5, 7) }),

        // Red
        (Coordinate::new(4, 4), Special { faction: faction::Color::Red, other: Coordinate::new(11, 11) }),
        (Coordinate::new(11, 11), Special { faction: faction::Color::Red, other: Coordinate::new(4, 4) }),

        // Orange
        (Coordinate::new(7, 5), Special { faction: faction::Color::Orange, other: Coordinate::new(8, 10) }),
        (Coordinate::new(8, 10), Special { faction: faction::Color::Orange, other: Coordinate::new(7, 5) }),

        // Slate
        (Coordinate::new(6, 6), Special { faction: faction::Color::Slate, other: Coordinate::new(9, 9) }),
        (Coordinate::new(9, 9), Special { faction: faction::Color::Slate, other: Coordinate::new(6, 6) }),

        // Yellow
        (Coordinate::new(5, 5), Special { faction: faction::Color::Yellow, other: Coordinate::new(10, 10) }),
        (Coordinate::new(10, 10), Special { faction: faction::Color::Yellow, other: Coordinate::new(5, 5) }),

        // Black
        (Coordinate::new(7, 8), Special { faction: faction::Color::Black, other: Coordinate::new(8, 7) }),
        (Coordinate::new(8, 7), Special { faction: faction::Color::Black, other: Coordinate::new(7, 8) }),
    ]);
}
