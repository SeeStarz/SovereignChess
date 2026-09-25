use crate::{
    Coordinate, GameState, MoveRich, PieceSimple,
    chess_move::NormalMove,
    direction,
    faction::Allegiance,
    logic::{self, move_generation::calculate::helper::try_add_move_check_special_tile_rules},
    piece,
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
            let destination = origin.offset(direction * distance);
            let Some(tile) = game_state.board.at(destination) else {
                break;
            };

            if let Some(victim) = tile.0 {
                if logic::allegiance(game_state, victim.faction) == Allegiance::Enemy {
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
            } else {
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
