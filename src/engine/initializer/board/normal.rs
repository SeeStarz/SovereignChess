use std::collections::HashMap;

use crate::engine::{Coordinate, PieceSimple, faction, piece};

pub fn generate() -> HashMap<Coordinate, PieceSimple> {
    HashMap::from([
        (
            Coordinate::new_unchecked(15, 8),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new_unchecked(0, 8),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new_unchecked(15, 7),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 7),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(7, 0),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 0),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 15),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(12, 15),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(3, 0),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(8, 15),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(7, 15),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(12, 0),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(3, 15),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 0),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(0, 15),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(8, 0),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new_unchecked(15, 4),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 11),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 4),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 11),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 13),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(14, 15),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(1, 0),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(4, 0),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(11, 15),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 2),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 2),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(4, 15),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(11, 0),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(14, 0),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(1, 15),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(0, 13),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new_unchecked(15, 6),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 9),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 6),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 9),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(6, 0),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 1),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 14),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(13, 15),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(2, 0),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(9, 15),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(6, 15),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(13, 0),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(2, 15),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 1),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(0, 14),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(9, 0),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new_unchecked(15, 5),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 10),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 5),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 10),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 12),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 14),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(1, 1),
            PieceSimple {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(5, 0),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(10, 15),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 3),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(15, 3),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(5, 15),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(10, 0),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(1, 14),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 1),
            PieceSimple {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(0, 12),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new_unchecked(14, 4),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 5),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 6),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 7),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 8),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 9),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 10),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 11),
            PieceSimple {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 4),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 5),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 6),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 7),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 8),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 9),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 10),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 11),
            PieceSimple {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 12),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 13),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(13, 14),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(12, 14),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(11, 14),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(10, 14),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(9, 14),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(8, 14),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(7, 14),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(6, 14),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(5, 14),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(4, 14),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(3, 14),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(2, 14),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 13),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 12),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 3),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(1, 2),
            PieceSimple {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(2, 1),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(3, 1),
            PieceSimple {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(4, 1),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(5, 1),
            PieceSimple {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(6, 1),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(7, 1),
            PieceSimple {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(8, 1),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(9, 1),
            PieceSimple {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(10, 1),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(11, 1),
            PieceSimple {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(12, 1),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(13, 1),
            PieceSimple {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 2),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new_unchecked(14, 3),
            PieceSimple {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
    ])
}
