use shared_model::definition::engine::tile::SpecialPair;

use crate::{
    PieceWithCoordinate,
    shared::{Coordinate, FactionId, PieceSimple, tile},
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileSimple(pub Option<PieceSimple>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileWithCoordinate {
    pub piece: Option<PieceWithCoordinate>,
    pub special_tile_pair: Option<tile::SpecialPair>,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecialLayout(pub Vec<tile::SpecialPair>);

impl SpecialLayout {
    pub fn new(inputs: Vec<SpecialPair>) -> Option<Self> {
        let faction_hash_set: HashSet<FactionId> = inputs.iter().map(|i| i.faction).collect();
        if faction_hash_set.len() != inputs.len() {
            return None;
        }

        let coordinates_hash_set: HashSet<Coordinate> =
            inputs.iter().flat_map(|i| i.coordinates).collect();
        if coordinates_hash_set.len() != 2 * inputs.len() {
            return None;
        }

        Some(Self(inputs))
    }

    pub fn all(&self) -> impl Iterator<Item = tile::SpecialPair> {
        self.0.iter().cloned()
    }

    pub fn at(&self, coordinate: Coordinate) -> Option<tile::SpecialPair> {
        self.0
            .iter()
            .find(|s| s.coordinates[0] == coordinate || s.coordinates[1] == coordinate)
            .cloned()
    }
}
