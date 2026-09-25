use crate::{Coordinate, FactionID, PieceRich, PieceSimple, PieceWithCoordinate};
use std::collections::HashSet;

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
    faction: FactionID,
    coordinate: Coordinate,
    other_coordinate: Coordinate,
}

impl Special {
    pub fn faction(&self) -> FactionID {
        self.faction
    }

    pub fn coordinate(&self) -> Coordinate {
        self.coordinate
    }

    pub fn other_coordinate(&self) -> Coordinate {
        self.other_coordinate
    }

    fn new_pair(coordinates: [Coordinate; 2], faction: FactionID) -> [Self; 2] {
        [
            Special {
                faction,
                coordinate: coordinates[0],
                other_coordinate: coordinates[1],
            },
            Special {
                faction,
                coordinate: coordinates[1],
                other_coordinate: coordinates[0],
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialLayoutInput {
    pub coordinates: [Coordinate; 2],
    pub faction: FactionID,
}

impl SpecialLayoutInput {
    pub fn new(coordinate1: Coordinate, coordinate2: Coordinate, faction: FactionID) -> Self {
        Self {
            coordinates: [coordinate1, coordinate2],
            faction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialLayout(Vec<Special>);

impl SpecialLayout {
    pub fn new(inputs: &[SpecialLayoutInput]) -> Option<Self> {
        let faction_hash_set: HashSet<FactionID> = inputs.iter().map(|i| i.faction).collect();
        if faction_hash_set.len() != inputs.len() {
            return None;
        }

        let coordinates_hash_set: HashSet<Coordinate> =
            inputs.iter().flat_map(|i| i.coordinates).collect();
        if coordinates_hash_set.len() != 2 * inputs.len() {
            return None;
        }

        let specials: Vec<Special> = inputs
            .iter()
            .flat_map(|i| Special::new_pair(i.coordinates, i.faction))
            .collect();
        Some(Self(specials))
    }

    pub fn all(&self) -> impl Iterator<Item = Special> {
        self.0.iter().cloned()
    }

    pub fn at(&self, coordinate: Coordinate) -> Option<Special> {
        self.0.iter().find(|s| s.coordinate == coordinate).cloned()
    }

    /// Panics
    /// If special provided isn't from this layout
    pub fn other(&self, special: Special) -> Special {
        if !self.0.contains(&special) {
            panic!("Provided special tile isn't from this layout")
        }

        *self
            .0
            .iter()
            .find(|s| s.coordinate == special.other_coordinate)
            .expect("Special tile has no pair")
    }
}
