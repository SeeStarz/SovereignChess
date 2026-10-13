use crate::definition::engine::{BoardView, FactionId, PlayerId, VariantData};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameStateView {
    pub board_view: BoardView,
    pub player_main_factions: HashMap<PlayerId, FactionId>,
    pub player_owned_factions: HashMap<PlayerId, FactionId>,
    pub owned_factions: HashMap<FactionId, FactionId>,
    pub factions: Vec<FactionId>,
    pub turn_to_play: PlayerId,
    pub player_count: u32,
    pub waiting_swap: bool,
    pub variant_data: VariantData,
}
