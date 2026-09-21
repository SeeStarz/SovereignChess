use crate::engine::{
    GameState,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    model::{
        Coordinate, MoveRich, PieceSimple, chess_move::NormalMove, direction, faction::Allegiance,
        piece, tile,
    },
};

/// Responsible for Queen, Rook, Bishop, and King moves
pub fn add_moves_naive(
    moves: &mut Vec<MoveRich>,
    game_state: &GameState,
    piece: PieceSimple,
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

            if let Some(victim) = game_state.c().board.at(destination) {
                if logic::faction::get_allegiance(game_state, victim.faction) == Allegiance::Enemy {
                    try_add_move_check_special_tile_rules(
                        moves,
                        game_state,
                        MoveRich::NormalMove(NormalMove {
                            origin,
                            destination,
                        }),
                        piece.faction,
                    );
                }
                break;
            } else if tile::Special::at(destination)
                .is_none_or(|s| game_state.c().board.at(s.coordinate).is_none())
            {
                try_add_move_check_special_tile_rules(
                    moves,
                    game_state,
                    MoveRich::NormalMove(NormalMove {
                        origin,
                        destination,
                    }),
                    piece.faction,
                );
            }
        }
    }
}
