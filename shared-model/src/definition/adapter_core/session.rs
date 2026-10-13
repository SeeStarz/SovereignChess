use crate::definition::{
    adapter_core::{Gesture, IntraGameStateView},
    engine::{GameStateView, MoveSimple},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Action {
    Move(MoveSimple),
    Gesture(Gesture),
    Swap(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameView {
    pub intra_game_state: IntraGameStateView,
    pub game_state: GameStateView,
}
