use crate::engine::{
    Coordinate, Gamestate, LegalMove,
    faction::Allegiance,
    legal_move::{
        NormalMove,
        calculate::{
            get_bishop_directions, get_queen_directions, get_rook_directions,
            try_add_move_check_special_tile_rules,
        },
    },
    piece::{self, Piece},
    tile,
};

/// Responsible for Queen, Rook, Bishop, and King moves
pub fn add_linear_moves_naive(
    moves: &mut Vec<LegalMove>,
    gamestate: &Gamestate,
    piece: Piece,
    origin: Coordinate,
) {
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

            if let Some(victim) = gamestate.c().board.at(destination) {
                if gamestate.get_allegiance(victim.faction) == Allegiance::Enemy {
                    try_add_move_check_special_tile_rules(
                        gamestate,
                        moves,
                        LegalMove::NormalMove(NormalMove {
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
                try_add_move_check_special_tile_rules(
                    gamestate,
                    moves,
                    LegalMove::NormalMove(NormalMove {
                        origin,
                        destination,
                    }),
                    piece.faction,
                );
            }
        }
    }
}
