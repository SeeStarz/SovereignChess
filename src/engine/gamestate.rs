use crate::engine::{
    logic,
    model::{Move, board::Board, faction, piece::PieceExternal},
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
pub struct CanonicalState {
    pub(in crate::engine) board: Board,
    pub(in crate::engine) player_colors: [faction::Color; 2],
    pub(in crate::engine) turn_to_play: TurnToPlay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedState {
    pub(in crate::engine) faction_owners: [Option<faction::Color>; 12],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gamestate {
    pub(in crate::engine) canonical: CanonicalState,
    pub(in crate::engine) derived: DerivedState,
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
            CanonicalState {
                board,
                player_colors,
                turn_to_play,
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
    fn get_faction_owners(&self) -> [Option<faction::Color>; 12] {
        logic::faction::get_faction_owners(self)
    }
}

impl DerivedState {
    fn new(state: &CanonicalState) -> Self {
        let faction_owners = state.get_faction_owners();
        Self { faction_owners }
    }
}
