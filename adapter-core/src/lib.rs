use std::collections::HashSet;

use engine::{
    Coordinate, FactionID, GameState, MoveRich, MoveSimple, PieceRich,
    chess_move::CastleRich,
    faction,
    logic::{self, board_at_rich},
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
    Defection(FactionID),
    Castle,
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
    Selected(SelectedState),
    Promotion(PromotionState),
    Done(GamestateChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectedState {
    BoardSelection(PieceRich),
    DefectionSelection(FactionID),
    CastleSelection,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromotionState {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

pub enum SelectedStageResult {
    Done(GamestateChange),
    Promotion(PromotionState),
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
    pub valid_defections: Vec<FactionID>,
    pub valid_castles: Vec<CastleRich>,
    pub grabbed_piece: Option<PieceRich>,
    pub selected_defection: Option<FactionID>,
    pub castle_selected: bool,
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
            (_, Gesture::MenuClick(MenuClick::Castle)) => {
                AdapterState::Selected(SelectedState::CastleSelection)
            }

            // Invalid pairings (e.g. Drop on PromotionStage)
            _ => AdapterState::Idle,
        }
    }

    pub fn hint(&self) -> UIHint {
        let grabbable_pieces: Vec<PieceRich> = valid_select_pieces(&self.game_state).collect();
        let valid_defections: Vec<FactionID> = valid_defections(&self.game_state).collect();

        let valid_castles: Vec<CastleRich> = self
            .game_state
            .moves()
            .into_iter()
            .filter_map(|m| {
                if let MoveRich::Castle(castle_move) = m {
                    Some(castle_move)
                } else {
                    None
                }
            })
            .collect();

        let grabbed_piece: Option<PieceRich> = match self.state.clone() {
            AdapterState::Selected(SelectedState::BoardSelection(piece)) => Some(piece),
            AdapterState::Promotion(promotion_data) => {
                board_at_rich(&self.game_state, promotion_data.origin)
            }
            AdapterState::Done(game_state_change) => ui_move_origin(game_state_change.applied_move)
                .and_then(|c| board_at_rich(&self.game_state, c)),
            _ => None,
        };

        let selected_defection: Option<FactionID> = match self.state.clone() {
            AdapterState::Selected(SelectedState::DefectionSelection(faction)) => Some(faction),
            AdapterState::Done(game_state_change) => {
                if let MoveRich::Defection(defection_move) = game_state_change.applied_move {
                    Some(defection_move.faction)
                } else {
                    None
                }
            }
            _ => None,
        };

        let castle_selected: bool = match self.state.clone() {
            AdapterState::Selected(SelectedState::CastleSelection) => true,
            AdapterState::Done(game_state_change) => {
                if let MoveRich::Castle(_castle_move) = game_state_change.applied_move {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        let valid_destinations: HashSet<Coordinate> = if let Some(piece) = grabbed_piece {
            valid_advance_from_board_selection(&self.game_state, piece.coordinate)
                .map(|r| match r {
                    SelectedStageResult::Done(game_state_change) => {
                        ui_move_destination(game_state_change.applied_move)
                    }
                    SelectedStageResult::Promotion(promotion_data) => promotion_data.destination,
                })
                .collect()
        } else if let Some(faction) = selected_defection {
            valid_advance_from_defection_selection(&self.game_state, faction)
                .map(|c| ui_move_destination(c.applied_move))
                .collect()
        } else if castle_selected {
            valid_castles
                .iter()
                .map(|c| c.king_move.destination)
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
                    Some(PromotionState {
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
            valid_castles,
            grabbed_piece,
            selected_defection,
            castle_selected,
            valid_destinations,
            promotion_options,
            game_state_change,
            state,
        }
    }

    fn try_advance_from_selected_do_board_click(
        &self,
        selected_data: SelectedState,
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
        selected_data: SelectedState,
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
        waiting_promotion_data: PromotionState,
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
            AdapterState::Selected(SelectedState::BoardSelection(piece))
        } else {
            AdapterState::Idle
        }
    }

    fn try_selected_do_defection_click(&self, faction: FactionID) -> AdapterState {
        if let Some(faction) = is_valid_idle_do_defection_click(&self.game_state, faction) {
            AdapterState::Selected(SelectedState::DefectionSelection(faction))
        } else {
            AdapterState::Idle
        }
    }
}

fn valid_select_pieces(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    let real_faction_owners = logic::real_faction_owners(game_state);
    game_state.pieces().filter(move |p| {
        real_faction_owners.get(&p.faction).cloned()
            == Some(logic::current_player_faction(game_state))
    })
}

fn valid_defections(game_state: &GameState) -> impl Iterator<Item = FactionID> {
    let real_faction_owners = logic::real_faction_owners(game_state);
    faction::ColorDefault::iter()
        .map(|f| FactionID::from(f))
        .filter(move |&f| {
            real_faction_owners.get(&f).cloned() == Some(logic::current_player_faction(game_state))
                && f != logic::current_player_faction(game_state)
        })
}

fn valid_advance_from_board_selection(
    game_state: &GameState,
    origin: Coordinate,
) -> impl Iterator<Item = SelectedStageResult> {
    game_state.moves().into_iter().filter_map(move |m| {
        let Some(move_origin) = ui_move_origin(m) else {
            return None;
        };

        if move_origin == origin {
            match m {
                MoveRich::Promotion(_) | MoveRich::RegimeChangePromotion(_) => {
                    Some(SelectedStageResult::Promotion(PromotionState {
                        origin,
                        destination: ui_move_destination(m),
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

fn valid_advance_from_defection_selection(
    game_state: &GameState,
    faction: FactionID,
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

fn valid_advance_from_castle_selection(
    game_state: &GameState,
) -> impl Iterator<Item = GamestateChange> {
    game_state.moves().into_iter().filter_map(move |m| {
        if let MoveRich::Castle(_castle_move) = m {
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
    promotion_stage_data: PromotionState,
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
    faction: FactionID,
) -> Option<FactionID> {
    valid_defections(game_state).find(|&f| f == faction)
}

fn is_valid_selected_do_board_click(
    game_state: &GameState,
    selected_data: SelectedState,
    destination: Coordinate,
) -> Option<SelectedStageResult> {
    match selected_data {
        SelectedState::BoardSelection(piece) => {
            valid_advance_from_board_selection(game_state, piece.coordinate).find(|r| match r {
                SelectedStageResult::Promotion(promotion_stage_data) => {
                    promotion_stage_data.origin == piece.coordinate
                        && promotion_stage_data.destination == destination
                }
                SelectedStageResult::Done(game_state_change) => {
                    ui_move_origin(game_state_change.applied_move) == Some(piece.coordinate)
                        && ui_move_destination(game_state_change.applied_move) == destination
                }
            })
        }

        SelectedState::DefectionSelection(faction) => {
            valid_advance_from_defection_selection(game_state, faction).find_map(|c| {
                if let MoveRich::Defection(defection_move) = c.applied_move {
                    if defection_move.faction == faction
                        && ui_move_destination(c.applied_move) == destination
                    {
                        Some(SelectedStageResult::Done(c))
                    } else {
                        None
                    }
                } else {
                    panic!(
                        "Got non-defection move from valid_advance_from_selected_defection_click"
                    )
                }
            })
        }

        SelectedState::CastleSelection => {
            valid_advance_from_castle_selection(game_state).find_map(|c| {
                if ui_move_destination(c.applied_move) == destination {
                    Some(SelectedStageResult::Done(c))
                } else {
                    None
                }
            })
        }
    }
}

fn is_valid_promotion_do_board_click(
    game_state: &GameState,
    promotion_stage_data: PromotionState,
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

fn ui_move_origin(chess_move: MoveRich) -> Option<Coordinate> {
    match chess_move {
        MoveRich::NormalMove(normal_move) => Some(normal_move.origin),
        MoveRich::Castle(_castle_move) => None,
        MoveRich::Defection(_defection_move) => None,
        MoveRich::RegimeChangePromotion(promotion_move) => Some(promotion_move.pawn_move.origin),
        MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move.origin),
    }
}

fn ui_move_destination(chess_move: MoveRich) -> Coordinate {
    match chess_move {
        MoveRich::NormalMove(normal_move) => normal_move.destination,
        MoveRich::Castle(castle_move) => castle_move.king_move.destination,
        MoveRich::Defection(defection_move) => {
            defection_move.destination.unwrap_or(defection_move.origin)
        }
        MoveRich::RegimeChangePromotion(promotion_move) => promotion_move.pawn_move.destination,
        MoveRich::Promotion(promotion_move) => promotion_move.normal_move.destination,
    }
}
