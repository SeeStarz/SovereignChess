use crate::engine::model::{Board, faction, tile::Special};

pub fn is_special_tile_occupiable(
    board: &Board,
    special: Special,
    faction: faction::Color,
) -> bool {
    if special.faction == faction {
        return false;
    }

    if let Some(_) = board.at(special.other().coordinate) {
        false
    } else {
        true
    }
}
