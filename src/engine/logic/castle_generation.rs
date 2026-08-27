use crate::engine::{
    logic,
    model::{Board, chess_move::Castle, chess_move::NormalMove, direction, piece},
};

pub fn generate(board: &Board) -> Vec<Castle> {
    let mut castles = Vec::new();
    for king in logic::board::pieces(board).filter(|p| p.piece_type == piece::King) {
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

                let king_end_coordinate = king
                    .coordinate
                    .offset(direction * 2)
                    .expect("Internal logic error at castle_generation");

                let king_move = NormalMove {
                    origin: king.coordinate,
                    destination: king_end_coordinate,
                };

                let rook_end_coordinate = king
                    .coordinate
                    .offset(direction)
                    .expect("Internal logic error at castle_generation");

                let rook_move = NormalMove {
                    origin: coordinate,
                    destination: rook_end_coordinate,
                };

                castles.push(Castle {
                    king_move,
                    rook_move,
                });
            }
        }
    }
    castles
}
