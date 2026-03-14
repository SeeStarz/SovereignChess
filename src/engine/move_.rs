use crate::engine::{
    coordinate::Direction,
    export::{Coordinate, Gamestate},
    piece::{self, Piece},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

// BE CAREFUL ABOUT THE ORDER
const DIRECTION_STORE: [Direction; 16] = [
    // Rook
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(0, -1),
    Direction::new(-1, 0),
    // Bishop
    Direction::new(1, 1),
    Direction::new(1, -1),
    Direction::new(-1, 1),
    Direction::new(-1, -1),
    // Knight
    Direction::new(2, 1),
    Direction::new(2, -1),
    Direction::new(1, 2),
    Direction::new(1, -2),
    Direction::new(-1, 2),
    Direction::new(-1, -2),
    Direction::new(-2, 1),
    Direction::new(-2, -1),
];

fn get_queen_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..8]
}

fn get_rook_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..4]
}

fn get_bishop_directions() -> &'static [Direction] {
    &DIRECTION_STORE[4..8]
}

fn get_knight_directions() -> &'static [Direction] {
    &DIRECTION_STORE[8..16]
}

impl Gamestate {
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        DIRECTION_STORE.iter();
        self.pieces().for_each(|p| {
            match p.piece_type {
                piece::King | piece::Queen | piece::Rook | piece::Bishop => {
                    self.add_linear_moves_naive(&mut moves, p.into(), p.coordinate);
                }
                _ => {
                    all_squares(&mut moves, p.coordinate);
                }
            };
        });
        moves
    }

    /// Responsible for Queen, Rook, Bishop, and King moves
    fn add_linear_moves_naive(&self, moves: &mut Vec<Move>, piece: Piece, origin: Coordinate) {
        let (directions, distance) = match piece.piece_type {
            piece::King => (get_queen_directions(), 1),
            piece::Queen => (get_queen_directions(), 8),
            piece::Rook => (get_rook_directions(), 8),
            piece::Bishop => (get_bishop_directions(), 8),
            incorrect_type => {
                panic!("Incorrect type passed: {:?}", incorrect_type)
            }
        };

        for &direction in directions {
            for distance in 1..=distance {
                let Some(destination) = origin.offset(direction * distance) else {
                    break;
                };

                moves.push(Move {
                    origin,
                    destination,
                });
            }
        }
    }
}

fn all_squares(moves: &mut Vec<Move>, origin: Coordinate) {
    for row in 0..16 {
        for col in 0..16 {
            moves.push(Move {
                origin,
                destination: Coordinate::new_unchecked(row, col),
            });
        }
    }
}
