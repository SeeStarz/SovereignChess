use crate::{Board, CastleSource, direction, piece};

pub fn generate(board: &Board) -> Vec<CastleSource> {
    let mut castles = Vec::new();
    let board_length = board.width().max(board.height());
    for king in board.pieces().filter(|p| p.piece_type == piece::King) {
        for &direction in direction::rook() {
            for distance in 2..board_length {
                let coordinate = king.coordinate.offset(direction * distance as i32);
                let Some(tile) = board.at(coordinate) else {
                    continue;
                };
                let Some(piece) = tile.0 else {
                    continue;
                };
                if piece.piece_type != piece::Rook {
                    continue;
                }

                castles.push(CastleSource {
                    king_coordinate: king.coordinate,
                    rook_coordinate: coordinate,
                });
            }
        }
    }
    castles
}
