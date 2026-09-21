use std::collections::HashSet;

use crate::engine::export::{
    Coordinate, GameState, MoveRich, MoveSimple, PieceRich, faction,
    logic::{self, board_at_external},
    piece,
};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gesture {
    Board(BoardGesture),
    MenuClick(MenuClick),
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuClick {
    Promotion(piece::Type),
    Defection(faction::Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardGesture {
    Click(Coordinate),
    Drop(Option<Coordinate>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GamestateChange {
    pub updated_game_state: GameState,
    pub applied_move: MoveRich,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdapterState {
    Idle,
    Selected(SelectedStage),
    Promotion(PromotionStage),
    Done(GamestateChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectedStage {
    BoardSelection(PieceRich),
    DefectionSelection(faction::Color),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromotionStage {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

pub enum SelectedStageResult {
    Done(GamestateChange),
    Promotion(PromotionStage),
}

impl From<SelectedStageResult> for AdapterState {
    fn from(selected_data: SelectedStageResult) -> Self {
        match selected_data {
            SelectedStageResult::Done(game_state_change) => Self::Done(game_state_change),
            SelectedStageResult::Promotion(promotion_stage_data) => {
                Self::Promotion(promotion_stage_data)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIState {
    Idle,
    Selected,
    Promotion,
    Done,
}

impl From<AdapterState> for UIState {
    fn from(state: AdapterState) -> Self {
        match state {
            AdapterState::Idle => UIState::Idle,
            AdapterState::Selected(_) => UIState::Selected,
            AdapterState::Promotion(_) => UIState::Promotion,
            AdapterState::Done(_) => UIState::Done,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UIHint {
    pub grabbable_pieces: Vec<PieceRich>,
    pub valid_defections: Vec<faction::Color>,
    pub grabbed_piece: Option<PieceRich>,
    pub selected_defection: Option<faction::Color>,
    pub valid_destinations: Vec<Coordinate>,
    pub promotion_options: Vec<piece::Type>,
    pub game_state_change: Option<GamestateChange>,
    pub state: UIState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adapter {
    game_state: GameState,
    state: AdapterState,
}

impl Adapter {
    pub fn new(game_state: GameState) -> Self {
        Self {
            state: AdapterState::Idle,
            game_state,
        }
    }

    pub fn game_state(&self) -> GameState {
        self.game_state.clone()
    }

    pub fn apply(&mut self, gesture: Gesture) {
        self.state = match (self.state.clone(), gesture) {
            // Freeze the state on Done
            (AdapterState::Done(game_state_change), _) => AdapterState::Done(game_state_change),

            // This is not really needed, but just so you get the intention
            (_, Gesture::Cancel) => AdapterState::Idle,

            // Try advancing with board click
            // If failed, try re-selecting
            // Else go back to Idle
            (
                AdapterState::Selected(selected_data),
                Gesture::Board(BoardGesture::Click(coordinate)),
            ) => self.try_advance_from_selected_do_board_click(selected_data, coordinate),

            // Try advancing with board drop
            // If failed immediately go back to Idle (no reselecting)
            (
                AdapterState::Selected(selected_data),
                Gesture::Board(BoardGesture::Drop(coordinate)),
            ) => self.try_advance_from_selected_do_board_drop(selected_data, coordinate),

            // Try advancing
            // If failed, try re-selecting
            // Else go back to idle
            (
                AdapterState::Promotion(promotion_stage_data),
                Gesture::MenuClick(MenuClick::Promotion(piece_type)),
            ) => self.try_advance_from_promotion_do_board_click(promotion_stage_data, piece_type),

            // From any non-done state re-selecting is allowed
            (_, Gesture::Board(BoardGesture::Click(coordinate))) => {
                self.try_selected_do_board_click(coordinate)
            }
            (_, Gesture::MenuClick(MenuClick::Defection(faction))) => {
                self.try_selected_do_defection_click(faction)
            }

            // Invalid pairings (e.g. Drop on PromotionStage)
            _ => AdapterState::Idle,
        }
    }

    pub fn hint(&self) -> UIHint {
        let grabbable_pieces: Vec<PieceRich> = valid_select_pieces(&self.game_state).collect();
        let valid_defections: Vec<faction::Color> = valid_defections(&self.game_state).collect();

        let grabbed_piece: Option<PieceRich> = match self.state.clone() {
            AdapterState::Selected(SelectedStage::BoardSelection(piece)) => Some(piece),
            AdapterState::Promotion(promotion_data) => {
                board_at_external(&self.game_state, promotion_data.origin)
            }
            AdapterState::Done(game_state_change) => {
                get_move_origin(game_state_change.applied_move)
                    .and_then(|c| board_at_external(&self.game_state, c))
            }
            _ => None,
        };

        let selected_defection: Option<faction::Color> = match self.state.clone() {
            AdapterState::Selected(SelectedStage::DefectionSelection(faction)) => Some(faction),
            AdapterState::Done(game_state_change) => {
                if let MoveRich::Defection(defection_move) = game_state_change.applied_move {
                    Some(defection_move.faction)
                } else {
                    None
                }
            }
            _ => None,
        };

        let valid_destinations: HashSet<Coordinate> = if let Some(piece) = grabbed_piece {
            valid_advance_from_selected_do_board_click(&self.game_state, piece.coordinate)
                .map(|r| match r {
                    SelectedStageResult::Done(game_state_change) => {
                        get_move_destination(&self.game_state, game_state_change.applied_move)
                    }
                    SelectedStageResult::Promotion(promotion_data) => promotion_data.destination,
                })
                .collect()
        } else if let Some(faction) = selected_defection {
            valid_advance_from_selected_do_defection_click(&self.game_state, faction)
                .map(|c| get_move_destination(&self.game_state, c.applied_move))
                .collect()
        } else {
            HashSet::new()
        };

        let valid_destinations: Vec<Coordinate> = valid_destinations.into_iter().collect();

        let promotion_options: Vec<piece::Type> = {
            let promotion_data = match self.state.clone() {
                AdapterState::Promotion(promotion_data) => Some(promotion_data),
                AdapterState::Done(game_state_change) => (match game_state_change.applied_move {
                    MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move),
                    MoveRich::RegimeChangePromotion(promotion_move) => {
                        Some(promotion_move.pawn_move)
                    }
                    _ => None,
                })
                .and_then(|m| {
                    Some(PromotionStage {
                        origin: m.origin,
                        destination: m.destination,
                    })
                }),
                _ => None,
            };
            if let Some(promotion_data) = promotion_data {
                valid_promotions(&self.game_state, promotion_data)
                    .map(|c| match c.applied_move {
                        MoveRich::Promotion(promotion_move) => promotion_move.piece_type,
                        MoveRich::RegimeChangePromotion(_promotion_move) => piece::King,
                        _ => panic!("Got non-promotion move from valid_promotions"),
                    })
                    .collect()
            } else {
                Vec::new()
            }
        };

        let game_state_change = if let AdapterState::Done(game_state_change) = self.state.clone() {
            Some(game_state_change)
        } else {
            None
        };

        let state = UIState::from(self.state.clone());

        UIHint {
            grabbable_pieces,
            valid_defections,
            grabbed_piece,
            selected_defection,
            valid_destinations,
            promotion_options,
            game_state_change,
            state,
        }
    }

    fn try_advance_from_selected_do_board_click(
        &self,
        selected_data: SelectedStage,
        coordinate: Coordinate,
    ) -> AdapterState {
        if let Some(selected_stage_result) =
            is_valid_selected_do_board_click(&self.game_state, selected_data, coordinate)
        {
            AdapterState::from(selected_stage_result)
        } else {
            self.try_selected_do_board_click(coordinate)
        }
    }

    fn try_advance_from_selected_do_board_drop(
        &self,
        selected_data: SelectedStage,
        coordinate: Option<Coordinate>,
    ) -> AdapterState {
        if let Some(coordinate) = coordinate
            && let Some(selected_stage_result) =
                is_valid_selected_do_board_click(&self.game_state, selected_data, coordinate)
        {
            AdapterState::from(selected_stage_result)
        } else {
            AdapterState::Idle
        }
    }

    fn try_advance_from_promotion_do_board_click(
        &self,
        waiting_promotion_data: PromotionStage,
        piece_type: piece::Type,
    ) -> AdapterState {
        if let Some(game_state_change) =
            is_valid_promotion_do_board_click(&self.game_state, waiting_promotion_data, piece_type)
        {
            AdapterState::Done(game_state_change)
        } else {
            AdapterState::Idle
        }
    }

    fn try_selected_do_board_click(&self, coordinate: Coordinate) -> AdapterState {
        if let Some(piece) = is_valid_idle_do_board_click(&self.game_state, coordinate) {
            AdapterState::Selected(SelectedStage::BoardSelection(piece))
        } else {
            AdapterState::Idle
        }
    }

    fn try_selected_do_defection_click(&self, faction: faction::Color) -> AdapterState {
        if let Some(faction) = is_valid_idle_do_defection_click(&self.game_state, faction) {
            AdapterState::Selected(SelectedStage::DefectionSelection(faction))
        } else {
            AdapterState::Idle
        }
    }
}

fn valid_select_pieces(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    game_state.pieces().filter(|p| {
        game_state.derived.real_faction_owners[p.faction as usize]
            == Some(logic::current_player_faction(game_state))
    })
}

fn valid_defections(game_state: &GameState) -> impl Iterator<Item = faction::Color> {
    faction::Color::iter().filter(|&f| {
        game_state.derived.real_faction_owners[f as usize]
            == Some(logic::current_player_faction(game_state))
            && f != logic::current_player_faction(game_state)
    })
}

fn valid_advance_from_selected_do_board_click(
    game_state: &GameState,
    origin: Coordinate,
) -> impl Iterator<Item = SelectedStageResult> {
    game_state.moves().into_iter().filter_map(move |m| {
        let Some(move_origin) = get_move_origin(m) else {
            return None;
        };

        if move_origin == origin {
            match m {
                MoveRich::Promotion(_) | MoveRich::RegimeChangePromotion(_) => {
                    Some(SelectedStageResult::Promotion(PromotionStage {
                        origin,
                        destination: get_move_destination(game_state, m),
                    }))
                }
                _ => Some(SelectedStageResult::Done(GamestateChange {
                    updated_game_state: game_state.apply_move(MoveSimple::from(m)),
                    applied_move: m,
                })),
            }
        } else {
            None
        }
    })
}

fn valid_advance_from_selected_do_defection_click(
    game_state: &GameState,
    faction: faction::Color,
) -> impl Iterator<Item = GamestateChange> {
    game_state.moves().into_iter().filter_map(move |m| {
        if let MoveRich::Defection(defection_move) = m
            && defection_move.faction == faction
        {
            Some(GamestateChange {
                updated_game_state: game_state.apply_move(MoveSimple::from(m)),
                applied_move: m,
            })
        } else {
            None
        }
    })
}

fn valid_promotions(
    game_state: &GameState,
    promotion_stage_data: PromotionStage,
) -> impl Iterator<Item = GamestateChange> {
    game_state
        .moves()
        .into_iter()
        .filter(move |m| match m {
            MoveRich::Promotion(m) => {
                m.normal_move.origin == promotion_stage_data.origin
                    && m.normal_move.destination == promotion_stage_data.destination
            }
            MoveRich::RegimeChangePromotion(m) => {
                m.pawn_move.origin == promotion_stage_data.origin
                    && m.pawn_move.destination == promotion_stage_data.destination
            }
            _ => false,
        })
        .map(|m| GamestateChange {
            updated_game_state: game_state.apply_move(MoveSimple::from(m)),
            applied_move: m,
        })
}

fn is_valid_idle_do_board_click(
    game_state: &GameState,
    coordinate: Coordinate,
) -> Option<PieceRich> {
    valid_select_pieces(game_state).find(|p| p.coordinate == coordinate)
}

fn is_valid_idle_do_defection_click(
    game_state: &GameState,
    faction: faction::Color,
) -> Option<faction::Color> {
    valid_defections(game_state).find(|&f| f == faction)
}

fn is_valid_selected_do_board_click(
    game_state: &GameState,
    selected_data: SelectedStage,
    destination: Coordinate,
) -> Option<SelectedStageResult> {
    match selected_data {
        SelectedStage::BoardSelection(piece) => {
            valid_advance_from_selected_do_board_click(game_state, piece.coordinate).find(|r| {
                match r {
                    SelectedStageResult::Promotion(promotion_stage_data) => {
                        promotion_stage_data.origin == piece.coordinate
                            && promotion_stage_data.destination == destination
                    }
                    SelectedStageResult::Done(game_state_change) => {
                        get_move_origin(game_state_change.applied_move) == Some(piece.coordinate)
                            && get_move_destination(game_state, game_state_change.applied_move)
                                == destination
                    }
                }
            })
        }

        SelectedStage::DefectionSelection(faction) => {
            valid_advance_from_selected_do_defection_click(game_state, faction).find_map(|c| {
                if let MoveRich::Defection(defection_move) = c.applied_move
                    && defection_move.faction == faction
                    && get_move_destination(game_state, c.applied_move) == destination
                {
                    Some(SelectedStageResult::Done(c))
                } else {
                    panic!(
                        "Got non-defection move from valid_advance_from_selected_defection_click"
                    )
                }
            })
        }
    }
}

fn is_valid_promotion_do_board_click(
    game_state: &GameState,
    promotion_stage_data: PromotionStage,
    piece_type: piece::Type,
) -> Option<GamestateChange> {
    valid_promotions(game_state, promotion_stage_data).find(|c| {
        (match c.applied_move {
            MoveRich::Promotion(promotion_move) => promotion_move.piece_type,
            MoveRich::RegimeChangePromotion(_promotion_move) => piece::King,
            _ => panic!("Got non-promotion move from valid_promotions"),
        }) == piece_type
    })
}

fn get_move_origin(chess_move: MoveRich) -> Option<Coordinate> {
    match chess_move {
        MoveRich::NormalMove(normal_move) => Some(normal_move.origin),
        MoveRich::Castle(castle_move) => Some(castle_move.king_move.origin),
        MoveRich::Defection(_defection_move) => None,
        MoveRich::RegimeChangePromotion(promotion_move) => Some(promotion_move.pawn_move.origin),
        MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move.origin),
    }
}

fn get_move_destination(game_state: &GameState, chess_move: MoveRich) -> Coordinate {
    match chess_move {
        MoveRich::NormalMove(normal_move) => normal_move.destination,
        MoveRich::Castle(castle_move) => castle_move.king_move.destination,
        MoveRich::Defection(defection_move) => defection_move
            .destination
            .unwrap_or_else(|| logic::find_current_player_king_assert(game_state).coordinate),
        MoveRich::RegimeChangePromotion(promotion_move) => promotion_move.pawn_move.destination,
        MoveRich::Promotion(promotion_move) => promotion_move.normal_move.destination,
    }
}
