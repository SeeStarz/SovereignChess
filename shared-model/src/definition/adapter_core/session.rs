use crate::definition::{
    adapter_core::{Gesture, IntraGameStateView},
    engine::{GameStateView, MoveSimple},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Move(MoveSimple),
    Gesture(Gesture),
    Swap(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameView {
    pub intra_game_state: IntraGameStateView,
    pub game_state: GameStateView,
}
