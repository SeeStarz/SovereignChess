use crate::engine::{Board, CastleSource, direction, logic, piece};

pub fn generate(board: &Board) -> Vec<CastleSource> {
    let mut castles = Vec::new();
    for king in logic::board_pieces(board).filter(|p| p.piece_type == piece::King) {
        for &direction in direction::rook() {
            for distance in 2..board.tiles.len() {
                let Some(coordinate) = king.coordinate.offset(direction * distance as i32) else {
                    break;
                };

                let Some(piece) = board.at(coordinate) else {
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
