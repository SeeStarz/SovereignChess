use std::collections::HashMap;

use crate::{Coordinate, FactionId, PieceSimple, faction, piece};

pub fn generate() -> HashMap<Coordinate, PieceSimple> {
    HashMap::from([
        (
            Coordinate::new(11, 6),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(0, 6),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(11, 5),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 5),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(11, 11),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(6, 11),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(5, 11),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 11),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 0),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(5, 0),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(6, 0),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(11, 0),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(11, 2),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 9),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 2),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 9),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(10, 11),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(9, 11),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(2, 11),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 11),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 0),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(2, 0),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(9, 0),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(10, 0),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 4),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(11, 7),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 4),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 7),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(11, 10),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(7, 11),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(4, 11),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 10),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 1),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(4, 0),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(7, 0),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(11, 1),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(11, 3),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(11, 8),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 3),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 8),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 10),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(8, 11),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(3, 11),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 10),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 1),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(3, 0),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(8, 0),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 1),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 2),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 3),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 4),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 5),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 6),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 7),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 8),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 9),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 2),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 3),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 4),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 5),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 6),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 7),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 8),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 9),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 10),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(8, 10),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(7, 10),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(6, 10),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 10),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 10),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 10),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 10),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 1),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 1),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 1),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 1),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(6, 1),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(7, 1),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(8, 1),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 1),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Pawn,
            },
        ),
    ])
}
