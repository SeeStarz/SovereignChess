use engine::{GameState, logic};
use shared_model::definition::engine::{
    BoardRow, BoardView, Coordinate, GameStateView, VariantData, VariantPreset,
};

pub fn game_state_from_variant_data(variant_data: VariantData) -> Option<GameState> {
    GameState::new(variant_data)
}

pub fn game_state_from_variant_preset(variant_preset: VariantPreset) -> GameState {
    GameState::new_from_preset(variant_preset)
}

pub fn view_game_state(game_state: &GameState) -> GameStateView {
    let special_tile_pairs = game_state.board.special_layout().all().collect();
    let pieces = logic::board_pieces_rich(game_state).collect();
    let tiles = logic::board_tiles_rich(game_state).collect();
    let promotion_area = game_state.variant_data.promotion_area;
    let width = game_state.variant_data.board_width;
    let height = game_state.variant_data.board_height;
    let rows = (0..height)
        .map(|r| {
            BoardRow(
                (0..width)
                    .map(|c| {
                        logic::board_tile_at_rich(game_state, Coordinate::new(r as i32, c as i32))
                            .expect("Variant data bounds inconsistent with actual board size")
                    })
                    .collect(),
            )
        })
        .collect();

    let board_view = BoardView {
        rows,
        tiles,
        pieces,
        special_tile_pairs,
        promotion_area,
        width,
        height,
    };

    let active_player_main_factions = game_state.player_main_factions.clone();
    let owned_factions = logic::real_faction_owners(game_state).into_iter().collect();
    let factions = logic::all_factions(&game_state.board).collect();
    let turn_to_play = game_state.turn_manager.current_player();
    let player_count = game_state.variant_data.player_main_factions.len() as u32;
    let variant_data = game_state.variant_data.clone();

    GameStateView {
        board_view,
        active_player_main_factions,
        owned_factions,
        factions,
        turn_to_play,
        player_count,
        variant_data,
    }
}
