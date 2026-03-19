use crate::engine::{Gamestate, LegalMove, gamestate::CanonicalState, piece};

pub fn apply_move(gamestate: &Gamestate, legal_move: LegalMove) -> CanonicalState {
    let mut board = gamestate.c().board;

    match legal_move {
        LegalMove::NormalMove(normal_move) => {
            let Some(piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            board.set_at(normal_move.origin, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        LegalMove::Promotion(promotion_move) => {
            let normal_move = promotion_move.normal_move;
            let Some(mut piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            assert!(piece.piece_type == piece::Pawn);
            piece.piece_type = promotion_move.piece_type;
            board.set_at(normal_move.origin, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        LegalMove::RegimeChangePromotion(promotion_move) => {
            let normal_move = promotion_move.normal_move;
            let Some(mut piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            assert!(piece.piece_type == piece::Pawn);

            let player_main_faction =
                gamestate.c().player_colors[gamestate.c().turn_to_play as usize];
            let king_coordinate = gamestate
                .pieces()
                .find_map(|p| {
                    if p.faction == player_main_faction && p.piece_type == piece::King {
                        Some(p.coordinate)
                    } else {
                        None
                    }
                })
                .expect(&format!(
                    "King is not found for faction {:?}",
                    player_main_faction
                ));

            piece.piece_type = piece::King;
            board.set_at(normal_move.origin, None);
            board.set_at(king_coordinate, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        _ => {
            panic!() // TODO: implement
        }
    };

    let player_colors = gamestate.c().player_colors;
    let turn_to_play = gamestate.c().turn_to_play.other();

    CanonicalState {
        board,
        player_colors,
        turn_to_play,
    }
}
