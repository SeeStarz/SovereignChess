use crate::{
    Board, Coordinate, FactionID, GameState, MoveRich, Vec2,
    chess_move::{NormalMove, Promotion, RegimeChangePromotionRich},
    faction::Allegiance,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    piece,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PawnMoveDirection {
    direction: Vec2,
    double_move: bool,
}
type PawnAttackDirection = Vec2;

pub fn add_moves_naive(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    faction: FactionID,
    origin: Coordinate,
) {
    let (move_directions, attack_directions) = calculate_pawn_directions(origin, &game_state.board);
    for pawn_direction in move_directions {
        let destination = origin.offset(pawn_direction.direction);
        let Some(tile) = game_state.board.at(destination) else {
            continue;
        };
        if tile.0.is_some() {
            continue;
        }

        try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
            moves,
            game_state,
            NormalMove {
                origin,
                destination,
            },
            faction,
        );

        if pawn_direction.double_move {
            let destination = origin.offset(pawn_direction.direction * 2);
            let Some(tile) = game_state.board.at(destination) else {
                continue;
            };
            if tile.0.is_some() {
                continue;
            }

            try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
                moves,
                game_state,
                NormalMove {
                    origin,
                    destination,
                },
                faction,
            );
        }
    }

    for direction in attack_directions {
        let destination = origin.offset(direction);
        let Some(tile) = game_state.board.at(destination) else {
            continue;
        };
        let Some(victim) = tile.0 else {
            continue;
        };
        if logic::allegiance(game_state, victim.faction) != Allegiance::Enemy {
            continue;
        }

        try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
            moves,
            game_state,
            NormalMove {
                origin,
                destination,
            },
            faction,
        );
    }
}

fn calculate_pawn_directions(
    origin: Coordinate,
    board: &Board,
) -> (Vec<PawnMoveDirection>, Vec<PawnAttackDirection>) {
    let move_directions = {
        let mut move_directions = Vec::new();
        if origin.row < board.promotion_area().top {
            move_directions.push(PawnMoveDirection {
                direction: Vec2::new(1, 0),
                double_move: origin.row < 2,
            });
        } else if origin.row > board.promotion_area().bottom {
            move_directions.push(PawnMoveDirection {
                direction: Vec2::new(-1, 0),
                double_move: origin.row >= board.height() as i32 - 2,
            });
        }

        if origin.col < board.promotion_area().left {
            move_directions.push(PawnMoveDirection {
                direction: Vec2::new(0, 1),
                double_move: origin.col < 2,
            });
        } else if origin.col > board.promotion_area().right {
            move_directions.push(PawnMoveDirection {
                direction: Vec2::new(0, -1),
                double_move: origin.col >= board.width() as i32 - 2,
            });
        }
        move_directions
    };

    let attack_directions = {
        let mut attack_directions = Vec::new();

        if move_directions.len() == 1 {
            let direction = move_directions[0].direction;
            if direction.row != 0 {
                attack_directions.push(direction + Vec2::new(0, 1));
                attack_directions.push(direction + Vec2::new(0, -1));
            } else {
                attack_directions.push(direction + Vec2::new(1, 0));
                attack_directions.push(direction + Vec2::new(-1, 0));
            }
        } else {
            // A pawn is somehow in the inner 2x2 ring if false
            assert!(move_directions.len() == 2);

            let dir1 = move_directions[0].direction;
            let dir2 = move_directions[1].direction;

            attack_directions.push(dir1 + dir2);
            attack_directions.push((-dir1) + dir2);
            attack_directions.push(dir1 + (-dir2));
        }
        attack_directions
    };

    (move_directions, attack_directions)
}

fn try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    normal_move: NormalMove,
    faction: FactionID,
) {
    if normal_move.destination.row >= game_state.board.promotion_area().top
        && normal_move.destination.row <= game_state.board.promotion_area().bottom
        && normal_move.destination.col >= game_state.board.promotion_area().left
        && normal_move.destination.col <= game_state.board.promotion_area().right
    {
        [piece::Queen, piece::Rook, piece::Bishop, piece::Knight]
            .into_iter()
            .for_each(|piece_type| {
                try_add_move_check_special_tile_rules(
                    moves,
                    game_state,
                    MoveRich::Promotion(Promotion {
                        normal_move,
                        piece_type,
                    }),
                    faction,
                )
            });
        try_add_move_check_special_tile_rules(
            moves,
            game_state,
            MoveRich::RegimeChangePromotion(RegimeChangePromotionRich {
                pawn_move: normal_move,
                king_origin: logic::find_current_player_king(game_state).map(|p| p.coordinate),
            }),
            faction,
        );
    } else {
        try_add_move_check_special_tile_rules(
            moves,
            game_state,
            MoveRich::NormalMove(normal_move),
            faction,
        );
    }
}
