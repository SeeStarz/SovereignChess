use crate::engine::{
    Gamestate,
    model::{
        Board, Coordinate,
        piece::{PieceExternal, PieceWithCoordinate},
    },
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
