#![allow(unused_imports)]
pub use crate::engine::gamestate::Gamestate;
pub use crate::engine::model::{
    Board, Coordinate, Direction, Move, Piece, PieceExternal, PieceWithCoordinate, Tile, board,
    chess_move, coordinate, direction, faction, piece, tile,
};

pub mod logic {
    pub use crate::engine::logic::board::{
        at_external as board_at_external, find_current_player_king,
        piece_externals as board_piece_externals, pieces as board_pieces,
    };
    pub use crate::engine::logic::faction::{
        current_player_faction, get_allegiance, get_real_faction_owners,
    };
    pub use crate::engine::logic::move_generation::{apply_move, calculate as calculate_move};
    pub use crate::engine::logic::special::is_special_tile_occupiable;
}
