use crate::{Coordinate, GameState, PieceRich, TileRich, logic, piece, tile};

pub fn tiles_rich(game_state: &GameState) -> impl Iterator<Item = TileRich> {
    game_state.board.tiles().map(|t| {
        let piece = at_rich(game_state, t.coordinate);
        TileRich {
            coordinate: t.coordinate,
            piece,
            special: tile::Special::at(t.coordinate).cloned(),
        }
    })
}

pub fn pieces_rich(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    tiles_rich(game_state).filter_map(|t| t.piece)
}

pub fn at_rich(game_state: &GameState, coordinate: Coordinate) -> Option<PieceRich> {
    let Some(tile) = game_state.board.at(coordinate) else {
        return None;
    };
    let Some(piece) = tile.0 else {
        return None;
    };
    let real_faction_owners = logic::real_faction_owners(game_state);
    Some(PieceRich::from_simple(
        piece,
        real_faction_owners.get(&piece.faction).cloned(),
        coordinate,
    ))
}

pub fn find_current_player_king(game_state: &GameState) -> Option<PieceRich> {
    let current_faction = logic::faction::current_player_faction(game_state);
    game_state
        .pieces()
        .find(|p| p.faction == current_faction && p.piece_type == piece::King)
}

pub fn find_current_player_king_assert(game_state: &GameState) -> PieceRich {
    find_current_player_king(game_state).expect(&format!(
        "King not found for player {:?}",
        game_state.turn_manager.current_player().0
    ))
}
