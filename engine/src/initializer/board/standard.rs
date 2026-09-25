use std::collections::HashMap;

use crate::{Coordinate, FactionId, PieceSimple, faction, piece};

pub fn generate() -> HashMap<Coordinate, PieceSimple> {
    HashMap::from([
        (
            Coordinate::new(15, 8),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(0, 8),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::King,
            },
        ),
        (
            Coordinate::new(15, 7),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 7),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(7, 0),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 0),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 15),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(12, 15),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(3, 0),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(8, 15),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(7, 15),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(12, 0),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(3, 15),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 0),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(0, 15),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(8, 0),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Queen,
            },
        ),
        (
            Coordinate::new(15, 4),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 11),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 4),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 11),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 13),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(14, 15),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 0),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(4, 0),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 15),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 2),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 2),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(4, 15),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(11, 0),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(14, 0),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(1, 15),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(0, 13),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Rook,
            },
        ),
        (
            Coordinate::new(15, 6),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 9),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 6),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 9),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(6, 0),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 1),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 14),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(13, 15),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(2, 0),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(9, 15),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(6, 15),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(13, 0),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(2, 15),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 1),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(0, 14),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(9, 0),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Bishop,
            },
        ),
        (
            Coordinate::new(15, 5),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 10),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 5),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 10),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 12),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 14),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 1),
            PieceSimple {
                faction: FactionId::from(faction::Slate),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(5, 0),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 15),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 3),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(15, 3),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(5, 15),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(10, 0),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(1, 14),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 1),
            PieceSimple {
                faction: FactionId::from(faction::Ash),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(0, 12),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Knight,
            },
        ),
        (
            Coordinate::new(14, 4),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 5),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 6),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 7),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 8),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 9),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 10),
            PieceSimple {
                faction: FactionId::from(faction::White),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 11),
            PieceSimple {
                faction: FactionId::from(faction::White),
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
            Coordinate::new(1, 10),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 11),
            PieceSimple {
                faction: FactionId::from(faction::Black),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 12),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 13),
            PieceSimple {
                faction: FactionId::from(faction::Pink),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(13, 14),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(12, 14),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(11, 14),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 14),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 14),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(8, 14),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(7, 14),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(6, 14),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 14),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 14),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 14),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 14),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 13),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 12),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 3),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(1, 2),
            PieceSimple {
                faction: FactionId::from(faction::Yellow),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(2, 1),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(3, 1),
            PieceSimple {
                faction: FactionId::from(faction::Orange),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(4, 1),
            PieceSimple {
                faction: FactionId::from(faction::Red),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(5, 1),
            PieceSimple {
                faction: FactionId::from(faction::Red),
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
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(9, 1),
            PieceSimple {
                faction: FactionId::from(faction::Violet),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(10, 1),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(11, 1),
            PieceSimple {
                faction: FactionId::from(faction::Navy),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(12, 1),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(13, 1),
            PieceSimple {
                faction: FactionId::from(faction::Cyan),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 2),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
        (
            Coordinate::new(14, 3),
            PieceSimple {
                faction: FactionId::from(faction::Green),
                piece_type: piece::Pawn,
            },
        ),
    ])
}
