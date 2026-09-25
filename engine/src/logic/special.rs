use crate::{Board, FactionId, tile::Special};

pub fn is_special_tile_occupiable(board: &Board, special: Special, faction: FactionId) -> bool {
    if special.faction() == faction {
        return false;
    }

    if let Some(_piece) = board
        .at(special.other_coordinate())
        .expect("Special tile out of bounds")
        .0
    {
        false
    } else {
        true
    }
}
