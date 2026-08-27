use crate::engine::{
    logic,
    model::{Coordinate, Move, board::Board, faction, piece::PieceExternal},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnToPlay {
    Player1 = 0,
    Player2 = 1,
}

impl TurnToPlay {
    pub fn other(&self) -> TurnToPlay {
        use TurnToPlay::*;
        match self {
            Player1 => Player2,
            Player2 => Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Castle {
    pub king_coordinate: Coordinate,
    pub rook_coordinate: Coordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalState {
    pub board: Board,
    pub player_colors: [faction::Color; 2],
    pub turn_to_play: TurnToPlay,
    pub remaining_castles: Vec<Castle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedState {
    pub real_faction_owners: [Option<faction::Color>; 12],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gamestate {
    pub canonical: CanonicalState,
    pub derived: DerivedState,
}

impl Gamestate {
    pub fn c(&self) -> &CanonicalState {
        &self.canonical
    }

    pub fn pieces(&self) -> impl Iterator<Item = PieceExternal> {
        logic::board::piece_externals(self)
    }

    pub fn new() -> Self {
        let canonical = {
            let board = Board::default();
            let player_colors = [faction::White, faction::Black];
            let turn_to_play = TurnToPlay::Player1;
            let remaining_castles = logic::castle_generation::generate(&board);
            CanonicalState {
                board,
                player_colors,
                turn_to_play,
                remaining_castles,
            }
        };

        let derived = DerivedState::new(&canonical);

        Self { canonical, derived }
    }

    pub fn moves(&self) -> Vec<Move> {
        logic::move_generation::calculate(self)
    }

    pub fn apply_move(&self, move_: Move) -> Self {
        let canonical = logic::move_generation::apply_move(self, move_);
        let derived = DerivedState::new(&canonical);

        Self { canonical, derived }
    }
}

impl CanonicalState {
    fn get_real_faction_owners(&self) -> [Option<faction::Color>; 12] {
        logic::faction::get_real_faction_owners(self)
    }
}

impl DerivedState {
    fn new(state: &CanonicalState) -> Self {
        let real_faction_owners = state.get_real_faction_owners();
        Self {
            real_faction_owners,
        }
    }
}
