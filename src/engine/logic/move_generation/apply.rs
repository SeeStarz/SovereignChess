use crate::engine::{
    Board, CastleSource, GameState, MoveSimple, PieceSimple, faction, game_state::CanonicalState,
    logic, piece, tile::Special,
};

pub fn apply_move(game_state: &GameState, chess_move: MoveSimple) -> CanonicalState {
    let mut board = game_state.c().board;

    move_pieces(game_state, &mut board, chess_move);

    let mut player_colors = game_state.c().player_colors;

    change_player_colors(game_state, &mut player_colors, chess_move);

    let mut remaining_castles = game_state.c().remaining_castles.clone();

    filter_remaining_castles(game_state, &mut remaining_castles, chess_move);

    let turn_to_play = game_state.c().turn_to_play.other();

    CanonicalState {
        board,
        player_colors,
        turn_to_play,
        remaining_castles,
    }
}

fn filter_remaining_castles(
    game_state: &GameState,
    remaining_castles: &mut Vec<CastleSource>,
    chess_move: MoveSimple,
) {
    let affected_coordinates = match chess_move {
        MoveSimple::NormalMove(normal_move) => vec![normal_move.origin, normal_move.destination],
        MoveSimple::Promotion(promotion_move) => vec![
            promotion_move.normal_move.origin,
            promotion_move.normal_move.destination,
        ],
        MoveSimple::RegimeChangePromotion(promotion_move) => {
            let player_main_faction =
                game_state.c().player_colors[game_state.c().turn_to_play as usize];

            let king_coordinate = game_state
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
                promotion_move.pawn_move.origin,
                promotion_move.pawn_move.destination,
                king_coordinate,
            ]
        }
        MoveSimple::Castle(castle_move) => {
            vec![
                castle_move.rook_move.origin,
                castle_move.rook_move.destination,
                castle_move.king_destination,
                logic::find_current_player_king_assert(game_state).coordinate,
            ]
        }
        MoveSimple::Defection(defection_move) => {
            if let Some(destination) = defection_move.destination {
                vec![
                    logic::find_current_player_king_assert(game_state).coordinate,
                    destination,
                ]
            } else {
                Vec::new()
            }
        }
    };

    remaining_castles.retain(|castle| {
        !affected_coordinates
            .iter()
            .any(|&c| castle.king_coordinate == c || castle.rook_coordinate == c)
    });
}

fn move_pieces(game_state: &GameState, board: &mut Board, chess_move: MoveSimple) {
    match chess_move {
        MoveSimple::NormalMove(normal_move) => {
            let Some(piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            board.set_at(normal_move.origin, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        MoveSimple::Promotion(promotion_move) => {
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
        MoveSimple::RegimeChangePromotion(promotion_move) => {
            let normal_move = promotion_move.pawn_move;
            let Some(mut piece) = board.at(normal_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    normal_move.origin
                );
            };
            assert!(piece.piece_type == piece::Pawn);

            let king_coordinate = logic::find_current_player_king_assert(game_state).coordinate;

            piece.piece_type = piece::King;
            board.set_at(normal_move.origin, None);
            board.set_at(king_coordinate, None);
            board.set_at(normal_move.destination, Some(piece));
        }
        MoveSimple::Castle(castle_move) => {
            let rook_move = castle_move.rook_move;
            let Some(rook) = board.at(rook_move.origin) else {
                panic!(
                    "Attempted to move nothing at position: {:?}",
                    rook_move.origin
                );
            };
            assert!(rook.piece_type == piece::Rook);
            let king_piece = logic::find_current_player_king_assert(game_state);
            let king_destination = castle_move.king_destination;

            board.set_at(rook_move.origin, None);
            board.set_at(king_piece.coordinate, None);
            board.set_at(rook_move.destination, Some(rook));
            board.set_at(king_destination, Some(PieceSimple::from(king_piece)));
        }
        MoveSimple::Defection(defection_move) => {
            let king_origin = logic::find_current_player_king_assert(game_state).coordinate;

            if let Some(destination) = defection_move.destination {
                assert!(
                    Special::at(king_origin).map(|s| s.faction)
                        == Some(logic::current_player_faction(game_state))
                );

                board.set_at(king_origin, None);
                board.set_at(
                    destination,
                    Some(PieceSimple {
                        faction: defection_move.faction,
                        piece_type: piece::King,
                    }),
                );
            } else {
                let king_coordinate = logic::find_current_player_king_assert(game_state).coordinate;

                assert!(
                    Special::at(king_coordinate).map(|s| s.faction)
                        != Some(logic::current_player_faction(game_state))
                );

                board.set_at(
                    king_coordinate,
                    Some(PieceSimple {
                        faction: defection_move.faction,
                        piece_type: piece::King,
                    }),
                )
            }
        }
    };
}

fn change_player_colors(
    game_state: &GameState,
    player_colors: &mut [faction::Color; 2],
    chess_move: MoveSimple,
) {
    match chess_move {
        MoveSimple::RegimeChangePromotion(promotion_move) => {
            let faction = logic::board_at_external(game_state, promotion_move.pawn_move.origin)
                .expect(&format!(
                    "Attempted to move nothing at position {:?}",
                    promotion_move.pawn_move.origin
                ))
                .faction;

            player_colors[game_state.c().turn_to_play as usize] = faction;
        }
        MoveSimple::Defection(defection_move) => {
            player_colors[game_state.c().turn_to_play as usize] = defection_move.faction;
        }
        _ => {}
    }
}
