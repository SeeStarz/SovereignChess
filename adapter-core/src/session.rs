use crate::gesture;
use engine::{GameSession, GameState, game_session, initializer, logic};
use shared_model::definition::{
    adapter_core::{GameView, SessionAction},
    engine::{
        BoardRow, BoardView, Coordinate, FactionId, GameStateView, MoveSimple, PlayerId,
        VariantData, VariantPreset,
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adapter {
    session: GameSession,
    gesture_adapter: gesture::Adapter,
    cached_game_view: GameView,
}

impl Adapter {
    pub fn new(variant_data: VariantData) -> Option<Self> {
        let session = GameSession::new(variant_data)?;

        assert_eq!(session.states().len(), 1);
        let game_state = Self::last_game_state(&session).clone();
        let gesture_adapter = gesture::Adapter::new(game_state);

        let intra_game_state_view = gesture_adapter.view();
        let game_state_view = Self::last_game_state_view(&session);
        let game_view = GameView {
            game_state: game_state_view,
            intra_game_state: intra_game_state_view,
        };

        Some(Self {
            session,
            gesture_adapter,
            cached_game_view: game_view,
        })
    }

    pub fn new_from_preset(variant_preset: VariantPreset) -> Self {
        let variant_data = match variant_preset {
            VariantPreset::Standard => initializer::variant::standard(),
            VariantPreset::Arena => initializer::variant::arena(),
        };

        Self::new(variant_data).expect(&format!("Variant {:?} is broken", variant_preset))
    }

    pub fn do_action(&mut self, action: SessionAction) -> bool {
        match action {
            SessionAction::Swap(swap) => {
                let command = game_session::Command::Swap(swap);
                let result = self.session.do_command(command);
                assert_eq!(
                    self.gesture_adapter.game_state(),
                    Self::last_game_state(&self.session)
                );
                self.cached_game_view = self.game_view_internal();
                result
            }
            SessionAction::Move(chess_move) => {
                let command = game_session::Command::Move(chess_move);
                let result = self.session.do_command(command);
                if result {
                    self.gesture_adapter =
                        gesture::Adapter::new(Self::last_game_state(&self.session).clone());
                    self.cached_game_view = self.game_view_internal();
                }
                result
            }
            SessionAction::Gesture(gesture) => {
                if self.session.waiting_swap_command() {
                    false
                } else {
                    self.gesture_adapter.apply(gesture);
                    self.cached_game_view = self.game_view_internal();
                    if let Some(move_made) = self.gesture_adapter.view().move_made {
                        self.do_action(SessionAction::Move(MoveSimple::from(move_made)));
                    }
                    true
                }
            }
        }
    }

    pub fn undo(&mut self) -> bool {
        let result = self.session.undo();
        if result {
            let game_state = Self::last_game_state(&self.session).clone();
            self.gesture_adapter = gesture::Adapter::new(game_state);
            self.cached_game_view = self.game_view_internal();
        }
        result
    }

    pub fn game_view(&self) -> GameView {
        self.cached_game_view.clone()
    }

    fn game_view_internal(&self) -> GameView {
        let game_state_view = Self::last_game_state_view(&self.session);
        let intra_game_state_view = self.gesture_adapter.view();
        GameView {
            intra_game_state: intra_game_state_view,
            game_state: game_state_view,
        }
    }

    fn last_game_state_view(session: &GameSession) -> GameStateView {
        let game_state = Self::last_game_state(session);
        let variant_data = session.variant_data();
        let seat_map = session.seat_map();

        let board_view = {
            let height = variant_data.board_height;
            let width = variant_data.board_width;
            let promotion_area = variant_data.promotion_area;
            let special_tile_pairs = variant_data.special_tile_pairs.clone();
            let pieces = logic::board_pieces_rich(game_state).collect();
            let rows = (0..height)
                .map(|r| {
                    BoardRow(
                        (0..width)
                            .map(|c| {
                                logic::board_tile_at_rich(
                                    game_state,
                                    Coordinate::new(r as i32, c as i32),
                                )
                                .expect(&format!("Tile out of bounds at r:{} c:{}", r, c))
                            })
                            .collect(),
                    )
                })
                .collect();
            let tiles = logic::board_tiles_rich(game_state).collect();
            BoardView {
                height,
                width,
                promotion_area,
                special_tile_pairs,
                pieces,
                rows,
                tiles,
            }
        };

        let player_count = variant_data.player_main_factions.len() as u32;
        let turn_to_play = *seat_map
            .get(&game_state.turn_manager.current_player())
            .expect(&format!(
                "Seat {:?} not found in seat_map",
                &game_state.turn_manager.current_player()
            ));
        let variant_data = variant_data.clone();
        let waiting_swap = session.waiting_swap_command();
        let factions = logic::all_factions(&game_state.board).collect();
        let player_main_factions: HashMap<PlayerId, FactionId> = game_state
            .player_main_factions
            .iter()
            .map(|(&s, &f)| {
                (
                    *seat_map
                        .get(&s)
                        .expect(&format!("Seat {:?} not found in seat_map", s)),
                    f,
                )
            })
            .collect();
        let owned_factions = logic::real_faction_owners(game_state);
        let player_owned_factions = owned_factions
            .iter()
            .filter_map(|(&faction_owner, &owned_faction)| {
                let player = player_main_factions
                    .iter()
                    .find_map(|(&s, &f)| if f == faction_owner { Some(s) } else { None });
                player.map(|p| (p, owned_faction))
            })
            .collect();

        GameStateView {
            board_view,
            player_count,
            turn_to_play,
            variant_data,
            waiting_swap,
            factions,
            player_main_factions,
            player_owned_factions,
            owned_factions,
        }
    }

    fn last_game_state(session: &GameSession) -> &GameState {
        session
            .states()
            .iter()
            .rev()
            .find_map(|s| match s {
                game_session::State::Game(game_state) => Some(game_state),
                _ => None,
            })
            .expect("No game_state found in session")
    }
}
