use crate::engine::{
    Gamestate,
    gamestate::CanonicalState,
    logic,
    model::{Board, Move, Piece, chess_move::Castle, faction, piece, tile::Special},
};

pub fn apply_move(gamestate: &Gamestate, legal_move: Move) -> CanonicalState {
    let mut board = gamestate.c().board;

    move_pieces(gamestate, &mut board, legal_move);

    let mut player_colors = gamestate.c().player_colors;

    change_player_colors(gamestate, &mut player_colors, legal_move);

    let mut remaining_castles = gamestate.c().remaining_castles.clone();

    filter_remaining_castles(gamestate, &mut remaining_castles, legal_move);

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
        Move::Castle(castle_move) => {
            vec![
                castle_move.rook_move.origin,
                castle_move.rook_move.destination,
                castle_move.king_move.origin,
                castle_move.king_move.destination,
            ]
        }
        Move::Defection(defection_move) => {
            if let Some(normal_move) = defection_move.normal_move {
                vec![normal_move.origin, normal_move.destination]
            } else {
                Vec::new()
            }
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

            let king_coordinate =
                logic::board::find_current_player_king_assert(gamestate).coordinate;

            piece.piece_type = piece::King;
            board.set_at(normal_move.origin, None);
            board.set_at(king_coordinate, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        Move::Castle(castle_move) => {
            let rook_move = castle_move.rook_move;
            let Some(rook) = board.at(rook_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    rook_move.origin
                );
            };
            assert!(rook.piece_type == piece::Rook);
            let king_move = castle_move.king_move;
            let Some(king) = board.at(king_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    king_move.origin
                );
            };
            assert!(king.piece_type == piece::King);

            board.set_at(rook_move.origin, None);
            board.set_at(king_move.origin, None);
            board.set_at(rook_move.destination, Some(rook));
            board.set_at(king_move.destination, Some(king));
        }
        Move::Defection(defection_move) => {
            if let Some(normal_move) = defection_move.normal_move {
                assert!(
                    normal_move.origin
                        == logic::board::find_current_player_king_assert(gamestate).coordinate
                );
                assert!(
                    Special::at(normal_move.origin).map(|s| s.faction)
                        == Some(logic::faction::current_player_faction(gamestate))
                );

                board.set_at(normal_move.origin, None);
                board.set_at(
                    normal_move.destination,
                    Some(Piece {
                        faction: defection_move.faction,
                        piece_type: piece::King,
                    }),
                );
            } else {
                let king_coordinate =
                    logic::board::find_current_player_king_assert(gamestate).coordinate;

                assert!(
                    Special::at(king_coordinate).map(|s| s.faction)
                        != Some(logic::faction::current_player_faction(gamestate))
                );

                board.set_at(
                    king_coordinate,
                    Some(Piece {
                        faction: defection_move.faction,
                        piece_type: piece::King,
                    }),
                )
            }
        }
    };
}

fn change_player_colors(
    gamestate: &Gamestate,
    player_colors: &mut [faction::Color; 2],
    legal_move: Move,
) {
    match legal_move {
        Move::RegimeChangePromotion(promotion_move) => {
            let faction = logic::board::at_external(gamestate, promotion_move.normal_move.origin)
                .expect(&format!(
                    "Attempted to move nothing at position {:?}",
                    promotion_move.normal_move.origin
                ))
                .faction;

            player_colors[gamestate.c().turn_to_play as usize] = faction;
        }
        Move::Defection(defection_move) => {
            player_colors[gamestate.c().turn_to_play as usize] = defection_move.faction;
        }
        _ => {}
    }
}
