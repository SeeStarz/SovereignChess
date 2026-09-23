use crate::{
    GameState, logic, {Board, Coordinate, PieceRich, PieceWithCoordinate, piece},
};

pub fn pieces(board: &Board) -> impl Iterator<Item = PieceWithCoordinate> {
    board.tiles.iter().enumerate().flat_map(move |(row, line)| {
        line.iter().enumerate().filter_map(move |(col, tile)| {
            tile.map(|piece| {
                PieceWithCoordinate::from_piece(
                    piece,
                    Coordinate::new_unchecked(row as i32, col as i32),
                )
            })
        })
    })
}

pub fn piece_externals(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    game_state
        .c()
        .board
        .tiles
        .iter()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.iter().enumerate().filter_map(move |(col, tile)| {
                tile.map(|piece| {
                    PieceRich::from_piece(
                        piece,
                        game_state.derived.real_faction_owners[piece.faction as usize],
                        Coordinate::new_unchecked(row as i32, col as i32),
                    )
                })
            })
        })
}

pub fn at_external(game_state: &GameState, coordinate: Coordinate) -> Option<PieceRich> {
    let Some(piece) = game_state.c().board.at(coordinate) else {
        return None;
    };
    Some(PieceRich::from_piece(
        piece,
        game_state.derived.real_faction_owners[piece.faction as usize],
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
        game_state.c().turn_to_play
    ))
}
