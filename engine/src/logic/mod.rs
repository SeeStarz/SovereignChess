mod board;
mod castle_generation;
mod faction;
mod move_generation;
mod special;

pub use board::{
    at_external as board_at_external, find_current_player_king, find_current_player_king_assert,
    piece_externals as board_piece_externals, pieces as board_pieces,
};
pub use castle_generation::generate as generate_castle;
pub use faction::{current_player_faction, get_allegiance, get_real_faction_owners};
pub use move_generation::{apply_move, calculate as calculate_move};
pub use special::is_special_tile_occupiable;
