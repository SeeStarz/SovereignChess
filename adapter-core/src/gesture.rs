use engine::{GameState, logic};
use shared_model::definition::{
    adapter_core::{BoardGesture, Gesture, IntraGameStateStage, IntraGameStateView, MenuClick},
    engine::{Coordinate, FactionId, MoveRich, PieceRich, Vec2, piece},
};
use std::{collections::HashSet, mem};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AdapterState {
    Idle,
    Selected(SelectionData),
    WaitingPromotion(WaitingPromotionData),
    Done(MoveRich),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SelectionData {
    BoardSelection(PieceRich),
    DefectionSelection(FactionId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WaitingPromotionData {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

enum SelectedStageResult {
    Done(MoveRich),
    Promotion(WaitingPromotionData),
}

impl From<SelectedStageResult> for AdapterState {
    fn from(selected_data: SelectedStageResult) -> Self {
        match selected_data {
            SelectedStageResult::Done(chess_move) => Self::Done(chess_move),
            SelectedStageResult::Promotion(waiting_promotion_data) => {
                Self::WaitingPromotion(waiting_promotion_data)
            }
        }
    }
}

impl From<AdapterState> for IntraGameStateStage {
    fn from(state: AdapterState) -> Self {
        match state {
            AdapterState::Idle => IntraGameStateStage::Idle,
            AdapterState::Selected(_selection_data) => IntraGameStateStage::Selected,
            AdapterState::WaitingPromotion(_waiting_promotion_data) => {
                IntraGameStateStage::Promotion
            }
            AdapterState::Done(_chess_move) => IntraGameStateStage::Done,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn apply(&mut self, gesture: Gesture) {
        let state = mem::replace(&mut self.state, AdapterState::Idle);
        self.state = self.apply_internal(state, gesture);
    }

    fn apply_internal(&self, state: AdapterState, gesture: Gesture) -> AdapterState {
        match state {
            AdapterState::Done(move_made) => AdapterState::Done(move_made),
            AdapterState::WaitingPromotion(waiting_promotion_data) => match gesture {
                Gesture::MenuClick(MenuClick::Promotion(piece_type)) => self
                    .advance_or_idle_from_promotion_do_board_click(
                        waiting_promotion_data,
                        piece_type,
                    ),
                Gesture::MenuClick(MenuClick::Defection(faction)) => {
                    self.advance_or_idle_from_idle_do_defection_click(faction)
                }
                _ => AdapterState::Idle,
            },
            AdapterState::Selected(selection_data) => match gesture {
                Gesture::Board(BoardGesture::Click(coordinate)) => self
                    .advance_or_selected_or_idle_from_selected_do_board_click(
                        selection_data,
                        coordinate,
                    ),
                Gesture::Board(BoardGesture::Drop(coordinate)) => {
                    self.advance_or_idle_from_selected_do_board_drop(selection_data, coordinate)
                }
                Gesture::MenuClick(MenuClick::Defection(faction)) => {
                    self.advance_or_idle_from_idle_do_defection_click(faction)
                }
                _ => AdapterState::Idle,
            },
            AdapterState::Idle => match gesture {
                Gesture::Board(BoardGesture::Click(coordinate)) => {
                    self.advance_or_idle_from_idle_do_board_click(coordinate, None)
                }
                Gesture::MenuClick(MenuClick::Defection(faction)) => {
                    self.advance_or_idle_from_idle_do_defection_click(faction)
                }
                _ => AdapterState::Idle,
            },
        }
    }

    pub fn view(&self) -> IntraGameStateView {
        let grabbable_pieces: Vec<PieceRich> = valid_select_pieces(&self.game_state).collect();
        let valid_defections: Vec<FactionId> = valid_defections(&self.game_state).collect();

        let grabbed_piece: Option<PieceRich> = match self.state.clone() {
            AdapterState::Selected(SelectionData::BoardSelection(piece)) => Some(piece),
            AdapterState::WaitingPromotion(promotion_data) => {
                logic::board_piece_at_rich(&self.game_state, promotion_data.origin)
            }
            AdapterState::Done(move_made) => ui_move_origin(move_made)
                .and_then(|c| logic::board_piece_at_rich(&self.game_state, c)),
            _ => None,
        };

        let selected_defection: Option<FactionId> = match self.state.clone() {
            AdapterState::Selected(SelectionData::DefectionSelection(faction)) => Some(faction),
            AdapterState::Done(move_made) => {
                if let MoveRich::Defection(defection_move) = move_made {
                    Some(defection_move.faction)
                } else {
                    None
                }
            }
            _ => None,
        };

        let valid_destinations: HashSet<Coordinate> = if let Some(piece) = grabbed_piece {
            valid_advance_from_board_selection(&self.game_state, piece.coordinate)
                .map(|r| match r {
                    SelectedStageResult::Done(move_made) => ui_move_destination(move_made),
                    SelectedStageResult::Promotion(waiting_promotion_data) => {
                        waiting_promotion_data.destination
                    }
                })
                .collect()
        } else if let Some(faction) = selected_defection {
            valid_advance_from_defection_selection(&self.game_state, faction)
                .map(|m| ui_move_destination(m))
                .collect()
        } else {
            HashSet::new()
        };

        let valid_destinations: Vec<Coordinate> = valid_destinations.into_iter().collect();

        let (promotion_options, promotion_at): (Vec<piece::Type>, Option<Coordinate>) = {
            let promotion_data = match self.state.clone() {
                AdapterState::WaitingPromotion(promotion_data) => Some(promotion_data),
                AdapterState::Done(move_made) => (match move_made {
                    MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move),
                    MoveRich::RegimeChangePromotion(promotion_move) => {
                        Some(promotion_move.pawn_move)
                    }
                    _ => None,
                })
                .and_then(|m| {
                    Some(WaitingPromotionData {
                        origin: m.origin,
                        destination: m.destination,
                    })
                }),
                _ => None,
            };

            if let Some(promotion_data) = promotion_data {
                let coordinate = promotion_data.destination;
                (
                    valid_promotions(&self.game_state, promotion_data)
                        .map(|m| match m {
                            MoveRich::Promotion(promotion_move) => promotion_move.piece_type,
                            MoveRich::RegimeChangePromotion(_promotion_move) => piece::King,
                            _ => panic!("Got non-promotion move from valid_promotions"),
                        })
                        .collect(),
                    Some(coordinate),
                )
            } else {
                (Vec::new(), None)
            }
        };

        let move_made = if let AdapterState::Done(move_made) = self.state.clone() {
            Some(move_made)
        } else {
            None
        };

        let state = IntraGameStateStage::from(self.state.clone());

        IntraGameStateView {
            grabbable_pieces,
            valid_defections,
            grabbed_piece,
            selected_defection,
            valid_destinations,
            promotion_options,
            promotion_at,
            move_made,
            state,
        }
    }

    fn advance_or_selected_or_idle_from_selected_do_board_click(
        &self,
        selected_data: SelectionData,
        coordinate: Coordinate,
    ) -> AdapterState {
        if let Some(selected_stage_result) =
            is_valid_selected_do_board_click(&self.game_state, selected_data, coordinate)
        {
            AdapterState::from(selected_stage_result)
        } else {
            match selected_data {
                SelectionData::BoardSelection(piece) => self
                    .advance_or_idle_from_idle_do_board_click(coordinate, Some(piece.coordinate)),
                SelectionData::DefectionSelection(_faction) => {
                    self.advance_or_idle_from_idle_do_board_click(coordinate, None)
                }
            }
        }
    }

    fn advance_or_idle_from_selected_do_board_drop(
        &self,
        selected_data: SelectionData,
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

    fn advance_or_idle_from_promotion_do_board_click(
        &self,
        waiting_promotion_data: WaitingPromotionData,
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

    fn advance_or_idle_from_idle_do_board_click(
        &self,
        coordinate: Coordinate,
        previous_coordinate: Option<Coordinate>,
    ) -> AdapterState {
        if Some(coordinate) == previous_coordinate {
            return AdapterState::Idle;
        }

        if let Some(piece) = is_valid_idle_do_board_click(&self.game_state, coordinate) {
            AdapterState::Selected(SelectionData::BoardSelection(piece))
        } else {
            AdapterState::Idle
        }
    }

    fn advance_or_idle_from_idle_do_defection_click(&self, faction: FactionId) -> AdapterState {
        if let Some(faction) = is_valid_idle_do_defection_click(&self.game_state, faction) {
            AdapterState::Selected(SelectionData::DefectionSelection(faction))
        } else {
            AdapterState::Idle
        }
    }
}

fn valid_select_pieces(game_state: &GameState) -> impl Iterator<Item = PieceRich> {
    let real_faction_owners = logic::real_faction_owners(game_state);
    logic::board_pieces_rich(game_state).filter(move |p| {
        real_faction_owners.get(&p.faction).cloned()
            == Some(logic::current_player_faction(game_state))
    })
}

fn valid_defections(game_state: &GameState) -> impl Iterator<Item = FactionId> {
    let real_faction_owners = logic::real_faction_owners(game_state);
    logic::all_factions(&game_state.board)
        .map(|f| FactionId::from(f))
        .filter(move |&f| {
            real_faction_owners.get(&f).cloned() == Some(logic::current_player_faction(game_state))
                && f != logic::current_player_faction(game_state)
        })
}

fn valid_advance_from_board_selection(
    game_state: &GameState,
    origin: Coordinate,
) -> impl Iterator<Item = SelectedStageResult> {
    logic::calculate_move(game_state)
        .into_iter()
        .filter_map(move |m| {
            let Some(move_origin) = ui_move_origin(m) else {
                return None;
            };

            if move_origin == origin {
                match m {
                    MoveRich::Promotion(_) | MoveRich::RegimeChangePromotion(_) => {
                        Some(SelectedStageResult::Promotion(WaitingPromotionData {
                            origin,
                            destination: ui_move_destination(m),
                        }))
                    }
                    _ => Some(SelectedStageResult::Done(m)),
                }
            } else {
                None
            }
        })
}

fn valid_advance_from_defection_selection(
    game_state: &GameState,
    faction: FactionId,
) -> impl Iterator<Item = MoveRich> {
    logic::calculate_move(game_state)
        .into_iter()
        .filter_map(move |m| {
            if let MoveRich::Defection(defection_move) = m
                && defection_move.faction == faction
            {
                Some(m)
            } else {
                None
            }
        })
}

fn valid_promotions(
    game_state: &GameState,
    promotion_stage_data: WaitingPromotionData,
) -> impl Iterator<Item = MoveRich> {
    logic::calculate_move(game_state)
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
}

fn is_valid_idle_do_board_click(
    game_state: &GameState,
    coordinate: Coordinate,
) -> Option<PieceRich> {
    valid_select_pieces(game_state).find(|p| p.coordinate == coordinate)
}

fn is_valid_idle_do_defection_click(
    game_state: &GameState,
    faction: FactionId,
) -> Option<FactionId> {
    valid_defections(game_state).find(|&f| f == faction)
}

fn is_valid_selected_do_board_click(
    game_state: &GameState,
    selected_data: SelectionData,
    destination: Coordinate,
) -> Option<SelectedStageResult> {
    match selected_data {
        SelectionData::BoardSelection(piece) => {
            valid_advance_from_board_selection(game_state, piece.coordinate).find(|r| match r {
                SelectedStageResult::Promotion(waiting_promotion_data) => {
                    waiting_promotion_data.origin == piece.coordinate
                        && waiting_promotion_data.destination == destination
                }
                &SelectedStageResult::Done(move_made) => {
                    ui_move_origin(move_made) == Some(piece.coordinate)
                        && ui_move_destination(move_made) == destination
                }
            })
        }

        SelectionData::DefectionSelection(faction) => {
            valid_advance_from_defection_selection(game_state, faction).find_map(|m| {
                if let MoveRich::Defection(defection_move) = m {
                    if defection_move.faction == faction && ui_move_destination(m) == destination {
                        Some(SelectedStageResult::Done(m))
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
    }
}

fn is_valid_promotion_do_board_click(
    game_state: &GameState,
    promotion_stage_data: WaitingPromotionData,
    piece_type: piece::Type,
) -> Option<MoveRich> {
    valid_promotions(game_state, promotion_stage_data).find(|m| {
        (match m {
            MoveRich::Promotion(promotion_move) => promotion_move.piece_type,
            MoveRich::RegimeChangePromotion(_promotion_move) => piece::King,
            _ => panic!("Got non-promotion move from valid_promotions"),
        }) == piece_type
    })
}

fn ui_move_origin(chess_move: MoveRich) -> Option<Coordinate> {
    match chess_move {
        MoveRich::NormalMove(normal_move) => Some(normal_move.origin),
        MoveRich::Castle(castle_move) => Some(castle_move.king_move.origin),
        MoveRich::Defection(_defection_move) => None,
        MoveRich::RegimeChangePromotion(promotion_move) => Some(promotion_move.pawn_move.origin),
        MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move.origin),
    }
}

fn ui_move_destination(chess_move: MoveRich) -> Coordinate {
    match chess_move {
        MoveRich::NormalMove(normal_move) => normal_move.destination,
        MoveRich::Castle(castle_move) => {
            if Vec2::from_coordinate_pair(
                castle_move.king_move.origin,
                castle_move.king_move.destination,
            )
            .manhattan_distance()
                == 1
            {
                castle_move.rook_move.origin
            } else {
                castle_move.king_move.destination
            }
        }
        MoveRich::Defection(defection_move) => {
            defection_move.destination.unwrap_or(defection_move.origin)
        }
        MoveRich::RegimeChangePromotion(promotion_move) => promotion_move.pawn_move.destination,
        MoveRich::Promotion(promotion_move) => promotion_move.normal_move.destination,
    }
}
