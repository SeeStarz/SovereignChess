use shared_model::definition::engine::Coordinate;

use crate::{Board, shared::FactionId};

/// # Panics
/// Panics if passed coordinate is invalid board coordinate
pub fn is_possibly_special_tile_occupiable(
    board: &Board,
    origin: Coordinate,
    destination: Coordinate,
    faction: FactionId,
) -> bool {
    let Some(special_tile_pair) = board
        .at(destination)
        .expect("Special tile check query out of bounds")
        .special_tile_pair
    else {
        return true;
    };

    if special_tile_pair.faction == faction {
        return false;
    }

    let other_coordinate = special_tile_pair
        .other_coordinate(destination)
        .expect("Coordinate inconsistency at is_special_tile_occupiable");

    if let Some(_piece) = board
        .at(other_coordinate)
        .expect("Special tile out of bounds")
        .piece
        && other_coordinate != origin
    {
        false
    } else {
        true
    }
}
