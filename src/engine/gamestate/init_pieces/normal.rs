use std::collections::HashMap;

use crate::engine::{Coordinate, faction, piece, piece::Piece};

pub fn generate() -> HashMap<Coordinate, Piece> {
    HashMap::from([
        (
            Coordinate::new_unchecked(15, 8),
            Piece {
                faction: faction::White,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new_unchecked(0, 8),
            Piece {
                faction: faction::Black,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new_unchecked(15, 7),
            Piece {
                faction: faction::White,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 7),
            Piece {
                faction: faction::Black,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(7, 0),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 0),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 15),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(12, 15),
            Piece {
                faction: faction::Red,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(3, 0),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(8, 15),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(7, 15),
            Piece {
                faction: faction::Green,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(12, 0),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(3, 15),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 0),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 15),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(8, 0),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 4),
            Piece {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 11),
            Piece {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 4),
            Piece {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 11),
            Piece {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 13),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(14, 15),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(1, 0),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(4, 0),
            Piece {
                faction: faction::Red,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(11, 15),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 2),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 2),
            Piece {
                faction: faction::Green,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(4, 15),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(11, 0),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(14, 0),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(1, 15),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 13),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 6),
            Piece {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 9),
            Piece {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 6),
            Piece {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 9),
            Piece {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(6, 0),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 1),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 14),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(13, 15),
            Piece {
                faction: faction::Red,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(2, 0),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(9, 15),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(6, 15),
            Piece {
                faction: faction::Green,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(13, 0),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(2, 15),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 1),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 14),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(9, 0),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 5),
            Piece {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 10),
            Piece {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 5),
            Piece {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 10),
            Piece {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 12),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 14),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(1, 1),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(5, 0),
            Piece {
                faction: faction::Red,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(10, 15),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 3),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 3),
            Piece {
                faction: faction::Green,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(5, 15),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(10, 0),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(1, 14),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 1),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 12),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 4),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 5),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 6),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 7),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 8),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 9),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 10),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 11),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 4),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 5),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 6),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 7),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 8),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 9),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 10),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 11),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 12),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 13),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(13, 14),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(12, 14),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(11, 14),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(10, 14),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(9, 14),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(8, 14),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(7, 14),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(6, 14),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(5, 14),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(4, 14),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(3, 14),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(2, 14),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 13),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 12),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 3),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 2),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(2, 1),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(3, 1),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(4, 1),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(5, 1),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(6, 1),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(7, 1),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(8, 1),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(9, 1),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(10, 1),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(11, 1),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(12, 1),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(13, 1),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 2),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 3),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
    ])
}
