use crate::definition::engine::{FactionId, TurnId, VariantData, board_view::BoardView};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameStateView {
    pub board_view: BoardView,
    pub active_player_main_factions: Vec<FactionId>,
    pub owned_factions: Vec<(FactionId, FactionId)>,
    pub factions: Vec<FactionId>,
    pub turn_to_play: TurnId,
    pub player_count: u32,
    pub variant_data: VariantData,
}
