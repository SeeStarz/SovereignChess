use crate::engine::{
    Gamestate, logic,
    model::{Board, Coordinate, PieceExternal, PieceWithCoordinate, piece},
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

pub fn piece_externals(gamestate: &Gamestate) -> impl Iterator<Item = PieceExternal> {
    gamestate
        .c()
        .board
        .tiles
        .iter()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.iter().enumerate().filter_map(move |(col, tile)| {
                tile.map(|piece| {
                    PieceExternal::from_piece(
                        piece,
                        gamestate.derived.real_faction_owners[piece.faction as usize],
                        Coordinate::new_unchecked(row as i32, col as i32),
                    )
                })
            })
        })
}

pub fn at_external(gamestate: &Gamestate, coordinate: Coordinate) -> Option<PieceExternal> {
    let Some(piece) = gamestate.c().board.at(coordinate) else {
        return None;
    };
    Some(PieceExternal::from_piece(
        piece,
        gamestate.derived.real_faction_owners[piece.faction as usize],
        coordinate,
    ))
}

pub fn find_current_player_king(gamestate: &Gamestate) -> PieceExternal {
    let current_faction = logic::faction::current_player_faction(gamestate);
    gamestate
        .pieces()
        .find(|p| p.faction == current_faction && p.piece_type == piece::King)
        .expect(&format!(
            "King not found for player {:?}",
            gamestate.c().turn_to_play
        ))
}
