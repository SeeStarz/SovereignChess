use crate::engine::{
    Gamestate,
    gamestate::CanonicalState,
    model::{Board, Move, chess_move::Castle, piece},
};

pub fn apply_move(gamestate: &Gamestate, legal_move: Move) -> CanonicalState {
    let mut board = gamestate.c().board;

    move_pieces(gamestate, &mut board, legal_move);

    let mut remaining_castles = gamestate.c().remaining_castles.clone();

    filter_remaining_castles(gamestate, &mut remaining_castles, legal_move);

    println!("{:?}\n{}", remaining_castles, remaining_castles.len());

    let player_colors = gamestate.c().player_colors;
    let turn_to_play = gamestate.c().turn_to_play.other();

    CanonicalState {
        board,
        player_colors,
        turn_to_play,
        remaining_castles,
    }
}

fn filter_remaining_castles(
    gamestate: &Gamestate,
    remaining_castles: &mut Vec<Castle>,
    legal_move: Move,
) {
    let affected_coordinates = match legal_move {
        Move::NormalMove(normal_move) => vec![normal_move.origin, normal_move.destination],
        Move::Promotion(promotion_move) => vec![
            promotion_move.normal_move.origin,
            promotion_move.normal_move.destination,
        ],
        Move::RegimeChangePromotion(promotion_move) => {
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

            vec![
                promotion_move.normal_move.origin,
                promotion_move.normal_move.destination,
                king_coordinate,
            ]
        }
        _ => {
            panic!() // TODO: implement
        }
    };

    remaining_castles.retain(|castle| {
        !affected_coordinates
            .iter()
            .any(|&c| castle.king_move.origin == c || castle.rook_move.origin == c)
    });
}

fn move_pieces(gamestate: &Gamestate, board: &mut Board, legal_move: Move) {
    match legal_move {
        Move::NormalMove(normal_move) => {
            let Some(piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            board.set_at(normal_move.origin, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        Move::Promotion(promotion_move) => {
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
        Move::RegimeChangePromotion(promotion_move) => {
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
}
