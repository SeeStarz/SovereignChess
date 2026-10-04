use crate::definition::engine::{Coordinate, FactionId, tile};

impl tile::SpecialPair {
    pub fn new(coordinate1: Coordinate, coordinate2: Coordinate, faction: FactionId) -> Self {
        Self {
            faction,
            coordinates: [coordinate1, coordinate2],
        }
    }

    pub fn other_coordinate(&self, coordinate: Coordinate) -> Option<Coordinate> {
        if self.coordinates[0] == coordinate {
            Some(self.coordinates[1])
        } else if self.coordinates[1] == coordinate {
            Some(self.coordinates[0])
        } else {
            None
        }
    }
}
