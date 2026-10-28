use crate::definition::engine::{Coordinate, FactionId, MoveRich, PieceRich, piece};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Gesture {
    Board(BoardGesture),
    MenuClick(MenuClick),
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MenuClick {
    Promotion(piece::Type),
    Defection(FactionId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BoardGesture {
    Click(Coordinate),
    Drop(Option<Coordinate>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IntraGameStateStage {
    Idle,
    Selected,
    Promotion,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntraGameStateView {
    pub grabbable_pieces: Vec<PieceRich>,
    pub valid_defections: Vec<FactionId>,
    pub grabbed_piece: Option<PieceRich>,
    pub selected_defection: Option<FactionId>,
    pub valid_destinations: Vec<Coordinate>,
    pub promotion_options: Vec<piece::Type>,
    pub promotion_at: Option<Coordinate>,
    pub move_made: Option<MoveRich>,
    pub state: IntraGameStateStage,
}
