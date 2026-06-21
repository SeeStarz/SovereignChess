use crate::engine::{
    Gamestate,
    logic::{
        self, move_generation::calculate::helper::try_add_legal_move_check_special_tile_rules,
    },
    model::{
        Coordinate, Piece,
        chess_move::{Move, NormalMove},
        direction,
        faction::Allegiance,
        piece, tile,
    },
};

/// Responsible for Queen, Rook, Bishop, and King moves
pub fn add_moves_naive(
    moves: &mut Vec<Move>,
    gamestate: &Gamestate,
    piece: Piece,
    origin: Coordinate,
) {
    let (directions, distance) = match piece.piece_type {
        piece::King => (direction::queen(), 1),
        piece::Queen => (direction::queen(), 8),
        piece::Rook => (direction::rook(), 8),
        piece::Bishop => (direction::bishop(), 8),
        incorrect_type => {
            panic!("Incorrect type passed: {:?}", incorrect_type)
        }
    };

    for &direction in directions {
        for distance in 1..=distance {
            let Some(destination) = origin.offset(direction * distance) else {
                break;
            };

            if let Some(victim) = gamestate.c().board.at(destination) {
                if logic::faction::get_allegiance(gamestate, victim.faction) == Allegiance::Enemy {
                    try_add_legal_move_check_special_tile_rules(
                        moves,
                        gamestate,
                        Move::NormalMove(NormalMove {
                            origin,
                            destination,
                        }),
                        piece.faction,
                    );
                }
                break;
            } else if tile::Special::at(destination)
                .is_none_or(|s| gamestate.c().board.at(s.coordinate).is_none())
            {
                try_add_legal_move_check_special_tile_rules(
                    moves,
                    gamestate,
                    Move::NormalMove(NormalMove {
                        origin,
                        destination,
                    }),
                    piece.faction,
                );
            }
        }
    }
}
