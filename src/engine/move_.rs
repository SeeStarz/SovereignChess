use crate::engine::{
    Coordinate, Gamestate,
    coordinate::Direction,
    faction::{self, Allegiance},
    piece::{self, Piece},
    tile,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub origin: Coordinate,
    pub destination: Coordinate,
}

// BE CAREFUL ABOUT THE ORDER
const DIRECTION_STORE: [Direction; 16] = [
    // Rook
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(0, -1),
    Direction::new(-1, 0),
    // Bishop
    Direction::new(1, 1),
    Direction::new(1, -1),
    Direction::new(-1, 1),
    Direction::new(-1, -1),
    // Knight
    Direction::new(2, 1),
    Direction::new(2, -1),
    Direction::new(1, 2),
    Direction::new(1, -2),
    Direction::new(-1, 2),
    Direction::new(-1, -2),
    Direction::new(-2, 1),
    Direction::new(-2, -1),
];

fn get_queen_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..8]
}

fn get_rook_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..4]
}

fn get_bishop_directions() -> &'static [Direction] {
    &DIRECTION_STORE[4..8]
}

fn get_knight_directions() -> &'static [Direction] {
    &DIRECTION_STORE[8..16]
}

impl Gamestate {
    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        self.pieces()
            .filter(|p| {
                self.derived.faction_owners[p.faction as usize]
                    == Some(self.c().player_colors[self.c().turn_to_play as usize])
            })
            .for_each(|p| {
                match p.piece_type {
                    piece::King | piece::Queen | piece::Rook | piece::Bishop => {
                        self.add_linear_moves_naive(&mut moves, p.into(), p.coordinate);
                    }
                    piece::Knight => {
                        self.add_knight_moves_naive(&mut moves, p.faction, p.coordinate);
                    }
                    piece::Pawn => {
                        self.add_pawn_moves_naive(&mut moves, p.faction, p.coordinate);
                    }
                };
            });
        moves
    }

    /// Responsible for Queen, Rook, Bishop, and King moves
    fn add_linear_moves_naive(&self, moves: &mut Vec<Move>, piece: Piece, origin: Coordinate) {
        let (directions, distance) = match piece.piece_type {
            piece::King => (get_queen_directions(), 1),
            piece::Queen => (get_queen_directions(), 8),
            piece::Rook => (get_rook_directions(), 8),
            piece::Bishop => (get_bishop_directions(), 8),
            incorrect_type => {
                panic!("Incorrect type passed: {:?}", incorrect_type)
            }
        };

        for &direction in directions {
            for distance in 1..=distance {
                let Some(destination) = origin.offset(direction * distance) else {
                    break;
                };

                if let Some(victim) = self.c().board.at(destination) {
                    if self.get_allegiance(victim.faction) == Allegiance::Enemy {
                        // It should be always allowed since the other tile must be empty
                        assert!(
                            self.check_special_tile_occupibility_rules_ok(
                                destination,
                                piece.faction
                            )
                        );

                        moves.push(Move {
                            origin,
                            destination,
                        });
                    }
                    break;
                } else if tile::Special::at(destination)
                    .is_none_or(|s| self.c().board.at(s.coordinate).is_none())
                {
                    if self.check_special_tile_occupibility_rules_ok(destination, piece.faction) {
                        moves.push(Move {
                            origin,
                            destination,
                        });
                    }
                }
            }
        }
    }

    fn add_knight_moves_naive(
        &self,
        moves: &mut Vec<Move>,
        faction: faction::Color,
        origin: Coordinate,
    ) {
        for &direction in get_knight_directions() {
            let Some(destination) = origin.offset(direction) else {
                continue;
            };
            if let Some(victim) = self.c().board.at(destination) {
                if self.get_allegiance(victim.faction) != Allegiance::Enemy {
                    continue;
                }
            }

            if self.check_special_tile_occupibility_rules_ok(destination, faction) {
                moves.push(Move {
                    origin,
                    destination,
                });
            }
        }
    }

    fn add_pawn_moves_naive(
        &self,
        moves: &mut Vec<Move>,
        faction: faction::Color,
        origin: Coordinate,
    ) {
        struct PawnMoveDirection {
            direction: Direction,
            double_move: bool,
        }

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

        for direction in move_directions {
            let Some(destination) = origin.offset(direction.direction) else {
                continue;
            };
            if self.c().board.at(destination).is_some() {
                continue;
            }

            if self.check_special_tile_occupibility_rules_ok(destination, faction) {
                moves.push(Move {
                    origin,
                    destination,
                });
            }

            if direction.double_move
                && let Some(destination) = origin.offset(direction.direction * 2)
            {
                if self.c().board.at(destination).is_some() {
                    continue;
                }

                if self.check_special_tile_occupibility_rules_ok(destination, faction) {
                    moves.push(Move {
                        origin,
                        destination,
                    });
                }
            }
        }

        for direction in attack_directions {
            let Some(destination) = origin.offset(direction) else {
                continue;
            };
            let Some(victim) = self.c().board.at(destination) else {
                continue;
            };
            if self.get_allegiance(victim.faction) != Allegiance::Enemy {
                continue;
            }

            if self.check_special_tile_occupibility_rules_ok(destination, faction) {
                moves.push(Move {
                    origin,
                    destination,
                });
            }
        }
    }
}

// For debug purposes, e.g. allowing a piece to move anywhere
#[allow(dead_code)]
fn all_squares(moves: &mut Vec<Move>, origin: Coordinate) {
    for row in 0..16 {
        for col in 0..16 {
            moves.push(Move {
                origin,
                destination: Coordinate::new_unchecked(row, col),
            });
        }
    }
}
