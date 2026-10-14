use crate::{
    GameState,
    shared::{MoveSimple, PlayerId, SeatId, VariantData, VariantPreset},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Game(GameState),
    Swap(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Move(MoveSimple),
    Swap(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSession {
    states: Vec<State>,
    variant_data: VariantData,
}

impl GameSession {
    pub fn new(variant_data: VariantData) -> Option<Self> {
        let state = GameState::new(&variant_data)?;
        let states = vec![State::Game(state)];

        Some(Self {
            states,
            variant_data,
        })
    }

    pub fn new_from_preset(variant_preset: VariantPreset) -> Self {
        Self::new(VariantData::from(variant_preset))
            .expect(&format!("Variant {:?} is broken", variant_preset))
    }

    pub fn do_command(&mut self, command: Command) -> bool {
        match command {
            Command::Swap(swap) => {
                if self.waiting_swap_command() {
                    self.states.push(State::Swap(swap));
                    true
                } else {
                    false
                }
            }
            Command::Move(chess_move) => {
                if self.waiting_swap_command() {
                    false
                } else {
                    let game_state = self
                        .states
                        .iter()
                        .rev()
                        .find_map(|c| match c {
                            State::Game(game_state) => Some(game_state),
                            _ => None,
                        })
                        .expect("No game_state found in GameSession");

                    if let Some(game_state) = game_state.apply_move(chess_move) {
                        self.states.push(State::Game(game_state));
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn waiting_swap_command(&self) -> bool {
        self.states.len() == 2
            && self.variant_data.switch_prompt_enabled
            && self.variant_data.player_main_factions.len() == 2 // Just after the first move is made
    }

    pub fn undo(&mut self) -> bool {
        if self.states.len() > 1 {
            self.states.pop();
            true
        } else {
            false
        }
    }

    pub fn states(&self) -> &Vec<State> {
        &self.states
    }

    pub fn variant_data(&self) -> &VariantData {
        &self.variant_data
    }

    pub fn seat_map(&self) -> HashMap<SeatId, PlayerId> {
        let mut seat_map: HashMap<SeatId, PlayerId> =
            (0..self.variant_data.player_main_factions.len())
                .map(|n| (SeatId(n as u32), PlayerId(n as u32)))
                .collect();

        if self
            .states
            .iter()
            .find_map(|s| match s {
                &State::Swap(swap) => Some(swap),
                _ => None,
            })
            .is_some_and(|swap| swap)
        {
            seat_map.insert(SeatId(0), PlayerId(1));
            seat_map.insert(SeatId(1), PlayerId(0));
        }

        seat_map
    }
}
