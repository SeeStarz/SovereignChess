mod board;
mod castle_generation;
mod faction;
mod move_generation;
mod special;

pub use board::{
    find_current_player_king, find_current_player_king_assert,
    piece_at_rich as board_piece_at_rich, pieces_rich as board_pieces_rich,
    tile_at_rich as board_tile_at_rich, tiles_rich as board_tiles_rich,
};
pub use castle_generation::generate as generate_castle;
pub use faction::{
    all as all_factions, allegiance, current_player_faction, is_faction_in_board,
    real_faction_owners,
};
pub use move_generation::{apply_move, calculate as calculate_move};
pub use special::is_possibly_special_tile_occupiable;
