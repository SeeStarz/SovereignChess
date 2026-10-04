use shared_model::definition::engine::PieceSimple;

use crate::{
    GameState, logic,
    shared::{Coordinate, PieceRich, TileRich, piece},
};

pub fn tiles_rich(game_state: &GameState) -> impl Iterator<Item = TileRich> {
    game_state.board.tiles().map(|t| {
        let piece = piece_at_rich(game_state, t.coordinate);
        TileRich {
            coordinate: t.coordinate,
            piece,
            special: game_state.board.special_layout().at(t.coordinate),
        }
    })
}

pub fn pieces_rich(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    tiles_rich(game_state).filter_map(|t| t.piece)
}

pub fn tile_at_rich(game_state: &GameState, coordinate: Coordinate) -> Option<TileRich> {
    let Some(tile) = game_state.board.at(coordinate) else {
        return None;
    };

    let real_faction_owners = logic::real_faction_owners(game_state);
    let piece = tile.piece.map(|p| {
        PieceRich::from_simple(
            PieceSimple::from(p),
            real_faction_owners.get(&p.faction).cloned(),
            coordinate,
        )
    });

    let special = game_state.board.special_layout().at(coordinate);

    Some(TileRich {
        coordinate,
        piece,
        special,
    })
}

pub fn piece_at_rich(game_state: &GameState, coordinate: Coordinate) -> Option<PieceRich> {
    let Some(tile) = game_state.board.at(coordinate) else {
        return None;
    };
    let Some(piece) = tile.piece else {
        return None;
    };
    let real_faction_owners = logic::real_faction_owners(game_state);
    Some(PieceRich::from_simple(
        PieceSimple::from(piece),
        real_faction_owners.get(&piece.faction).cloned(),
        coordinate,
    ))
}

pub fn find_current_player_king(game_state: &GameState) -> Option<PieceRich> {
    let current_faction = logic::faction::current_player_faction(game_state);
    logic::board_pieces_rich(game_state)
        .find(|p| p.faction == current_faction && p.piece_type == piece::King)
}

pub fn find_current_player_king_assert(game_state: &GameState) -> PieceRich {
    find_current_player_king(game_state).expect(&format!(
        "King not found for player {:?}",
        game_state.turn_manager.current_player().0
    ))
}
