use crate::engine::{
    gamestate::Castle,
    logic,
    model::{Board, direction, piece},
};

pub fn generate(board: &Board) -> Vec<Castle> {
    let mut castles = Vec::new();
    for king in logic::board::pieces(board).filter(|p| p.piece_type == piece::King) {
        for &direction in direction::rook() {
            for distance in 1..board.tiles.len() {
                let Some(coordinate) = king.coordinate.offset(direction * distance as i32) else {
                    break;
                };

                let Some(piece) = board.at(coordinate) else {
                    continue;
                };

                if piece.piece_type == piece::Rook {
                    castles.push(Castle {
                        king_coordinate: king.coordinate,
                        rook_coordinate: coordinate,
                    });
                }
            }
        }
    }
    castles
}
