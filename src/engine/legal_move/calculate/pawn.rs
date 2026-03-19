use crate::engine::{
    Coordinate, Gamestate, LegalMove,
    coordinate::Direction,
    faction::{self, Allegiance},
    legal_move::{
        NormalMove, Promotion, RegimeChangePromotion,
        calculate::try_add_legal_move_check_special_tile_rules,
    },
    piece,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PawnMoveDirection {
    direction: Direction,
    double_move: bool,
}
type PawnAttackDirection = Direction;

fn calculate_pawn_directions(
    origin: Coordinate,
) -> (Vec<PawnMoveDirection>, Vec<PawnAttackDirection>) {
    let move_directions = {
        let mut move_directions = Vec::new();
        if origin.row() < 7 {
            move_directions.push(PawnMoveDirection {
                direction: Direction::new(1, 0),
                double_move: origin.row() < 2,
            });
        } else if origin.row() > 8 {
            move_directions.push(PawnMoveDirection {
                direction: Direction::new(-1, 0),
                double_move: origin.row() > 13,
            });
        }

        if origin.col() < 7 {
            move_directions.push(PawnMoveDirection {
                direction: Direction::new(0, 1),
                double_move: origin.col() < 2,
            });
        } else if origin.col() > 8 {
            move_directions.push(PawnMoveDirection {
                direction: Direction::new(0, -1),
                double_move: origin.col() > 13,
            });
        }
        move_directions
    };

    let attack_directions = {
        let mut attack_directions = Vec::new();

        if move_directions.len() == 1 {
            let direction = move_directions[0].direction;
            if direction.row != 0 {
                attack_directions.push(direction + Direction::new(0, 1));
                attack_directions.push(direction + Direction::new(0, -1));
            } else {
                attack_directions.push(direction + Direction::new(1, 0));
                attack_directions.push(direction + Direction::new(-1, 0));
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
    moves: &mut Vec<LegalMove>,
    gamestate: &Gamestate,
    normal_move: NormalMove,
    faction: faction::Color,
) {
    if normal_move.destination.row() > 5
        && normal_move.destination.row() < 10
        && normal_move.destination.col() > 5
        && normal_move.destination.col() < 10
    {
        [piece::Queen, piece::Rook, piece::Bishop, piece::Knight]
            .into_iter()
            .for_each(|piece_type| {
                try_add_legal_move_check_special_tile_rules(
                    moves,
                    gamestate,
                    LegalMove::Promotion(Promotion {
                        normal_move,
                        piece_type,
                    }),
                    faction,
                )
            });
        try_add_legal_move_check_special_tile_rules(
            moves,
            gamestate,
            LegalMove::RegimeChangePromotion(RegimeChangePromotion { normal_move }),
            faction,
        );
    } else {
        try_add_legal_move_check_special_tile_rules(
            moves,
            gamestate,
            LegalMove::NormalMove(normal_move),
            faction,
        );
    }
}

pub fn add_pawn_moves_naive(
    moves: &mut Vec<LegalMove>,
    gamestate: &Gamestate,
    faction: faction::Color,
    origin: Coordinate,
) {
    let (move_directions, attack_directions) = calculate_pawn_directions(origin);
    for direction in move_directions {
        let Some(destination) = origin.offset(direction.direction) else {
            continue;
        };
        if gamestate.c().board.at(destination).is_some() {
            continue;
        }

        try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
            moves,
            gamestate,
            NormalMove {
                origin,
                destination,
            },
            faction,
        );

        if direction.double_move {
            let Some(destination) = origin.offset(direction.direction * 2) else {
                continue;
            };
            if gamestate.c().board.at(destination).is_some() {
                continue;
            }

            try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
                moves,
                gamestate,
                NormalMove {
                    origin,
                    destination,
                },
                faction,
            );
        }
    }

    for direction in attack_directions {
        let Some(destination) = origin.offset(direction) else {
            continue;
        };
        let Some(victim) = gamestate.c().board.at(destination) else {
            continue;
        };
        if gamestate.get_allegiance(victim.faction) != Allegiance::Enemy {
            continue;
        }

        try_add_pawn_move_with_possibly_promotion_check_special_tile_rules(
            moves,
            gamestate,
            NormalMove {
                origin,
                destination,
            },
            faction,
        );
    }
}
