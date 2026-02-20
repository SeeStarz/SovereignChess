use std::collections::HashMap;

use super::super::super::{Coordinate, faction, piece, piece::Piece};

pub fn generate() -> HashMap<Coordinate, Piece> {
    HashMap::from([
        (
            Coordinate::new(15, 8),
            Piece {
                faction: faction::White,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(0, 8),
            Piece {
                faction: faction::Black,
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(15, 7),
            Piece {
                faction: faction::White,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 7),
            Piece {
                faction: faction::Black,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(7, 0),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 0),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 15),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(12, 15),
            Piece {
                faction: faction::Red,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(3, 0),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(8, 15),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(7, 15),
            Piece {
                faction: faction::Green,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(12, 0),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(3, 15),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 0),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 15),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(8, 0),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 4),
            Piece {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 11),
            Piece {
                faction: faction::White,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 4),
            Piece {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 11),
            Piece {
                faction: faction::Black,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 13),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(14, 15),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 0),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(4, 0),
            Piece {
                faction: faction::Red,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 15),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 2),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 2),
            Piece {
                faction: faction::Green,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(4, 15),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 0),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(14, 0),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 15),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 13),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 6),
            Piece {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 9),
            Piece {
                faction: faction::White,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 6),
            Piece {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 9),
            Piece {
                faction: faction::Black,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(6, 0),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 1),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 14),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(13, 15),
            Piece {
                faction: faction::Red,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(2, 0),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(9, 15),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(6, 15),
            Piece {
                faction: faction::Green,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(13, 0),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(2, 15),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 1),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 14),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(9, 0),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 5),
            Piece {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 10),
            Piece {
                faction: faction::White,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 5),
            Piece {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 10),
            Piece {
                faction: faction::Black,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 12),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 14),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 1),
            Piece {
                faction: faction::Slate,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(5, 0),
            Piece {
                faction: faction::Red,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 15),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 3),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 3),
            Piece {
                faction: faction::Green,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(5, 15),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 0),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 14),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 1),
            Piece {
                faction: faction::Ash,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 12),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 4),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 5),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 6),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 7),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 8),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 9),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 10),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 11),
            Piece {
                faction: faction::White,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 4),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 5),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 6),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 7),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 8),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 9),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 10),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 11),
            Piece {
                faction: faction::Black,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 12),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 13),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(13, 14),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(12, 14),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(11, 14),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 14),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 14),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(8, 14),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(7, 14),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(6, 14),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 14),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 14),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 14),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 14),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 13),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 12),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 3),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 2),
            Piece {
                faction: faction::Yellow,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 1),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 1),
            Piece {
                faction: faction::Orange,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 1),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 1),
            Piece {
                faction: faction::Red,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(6, 1),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(7, 1),
            Piece {
                faction: faction::Pink,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(8, 1),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 1),
            Piece {
                faction: faction::Violet,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 1),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(11, 1),
            Piece {
                faction: faction::Navy,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(12, 1),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(13, 1),
            Piece {
                faction: faction::Cyan,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 2),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 3),
            Piece {
                faction: faction::Green,
                piece_type: piece::Pawn,
            },
        ),
    ])
}
