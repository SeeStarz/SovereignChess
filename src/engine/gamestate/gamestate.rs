use crate::engine::{
    Coordinate, NormalMove, board::Board, faction, gamestate::init_pieces, piece::PieceExternal,
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
        self.c()
            .board
            .tiles
            .iter()
            .enumerate()
            .flat_map(move |(row, line)| {
                line.iter().enumerate().filter_map(move |(col, tile)| {
                    tile.map(|piece| {
                        PieceExternal::from_piece(
                            piece,
                            self.derived.faction_owners[piece.faction as usize],
                            // None,
                            Coordinate::new_unchecked(row as i32, col as i32),
                        )
                    })
                })
            })
    }

    pub fn new() -> Self {
        let canonical = {
            let mut board = Board::empty();
            for (coordinate, piece) in init_pieces::normal() {
                assert!(board.at(coordinate).is_none());
                board.set_at(coordinate, Some(piece));
            }
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

    // TODO: Proper move logic
    pub fn apply_move(&self, move_: NormalMove) -> Self {
        let canonical = {
            let mut board = self.c().board;
            if let Some(piece) = board.at(move_.origin) {
                board.set_at(move_.origin, None);
                board.set_at(move_.destination, Some(piece));
            }
            let player_colors = self.c().player_colors;
            let turn_to_play = self.c().turn_to_play.other();
            CanonicalState {
                board,
                player_colors,
                turn_to_play,
            }
        };

        let derived = DerivedState::new(&canonical);

        Self { canonical, derived }
    }
}

impl DerivedState {
    fn new(state: &CanonicalState) -> Self {
        let faction_owners = state.get_faction_owners();
        Self { faction_owners }
    }
}
