use crate::engine::{
    Gamestate, logic,
    model::{
        Coordinate, Direction,
        chess_move::{Castle, Move, NormalMove},
        faction, tile,
    },
};

pub fn add_moves_naive(moves: &mut Vec<Move>, gamestate: &Gamestate) {
    for _castle_move in gamestate.canonical.remaining_castles.iter() {
        let castle_move = Castle {
            king_move: NormalMove {
                origin: Coordinate::new_unchecked(0, 0),
                destination: Coordinate::new_unchecked(0, 0),
            },
            rook_move: NormalMove {
                origin: Coordinate::new_unchecked(0, 0),
                destination: Coordinate::new_unchecked(0, 0),
            },
        };

        if !check_pieces_allied(gamestate, castle_move) {
            continue;
        }

        if !check_path_clear(gamestate, castle_move) {
            continue;
        }

        // Because this isn't even supposed to happen and
        // the helper faction checkfunction is unable to proces this
        if !check_no_special_tile(castle_move) {
            continue;
        }

        moves.push(Move::Castle(castle_move));
    }
}

fn check_pieces_allied(gamestate: &Gamestate, castle_move: Castle) -> bool {
    let Some(king_piece) = gamestate.c().board.at(castle_move.king_move.origin) else {
        panic!(
            "Desync with remaining castles. Expected to find king with castle {:#?}",
            castle_move
        )
    };
    let Some(rook_piece) = gamestate.c().board.at(castle_move.rook_move.origin) else {
        panic!(
            "Desync with remaining castles. Expected to find king with castle {:#?}",
            castle_move
        )
    };
    if logic::faction::get_allegiance(gamestate, king_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    if logic::faction::get_allegiance(gamestate, rook_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    true
}

fn check_path_clear(gamestate: &Gamestate, castle_move: Castle) -> bool {
    let rook_offset = Direction::from_coordinate_pair(
        castle_move.rook_move.origin,
        castle_move.rook_move.destination,
    );
    let move_distance = rook_offset.manhattan_distance();
    let rook_direction = Direction::new(
        rook_offset.row / move_distance as i32,
        rook_offset.col / move_distance as i32,
    );

    // Ensure up until the destination the tiles are empty
    for distance in 1..=move_distance {
        let Some(coordinate) = castle_move
            .rook_move
            .origin
            .offset(rook_direction * distance as i32)
        else {
            panic!("Castling out of bounds")
        };

        if let Some(_piece) = gamestate.c().board.at(coordinate) {
            return false;
        }
    }
    true
}

fn check_no_special_tile(castle_move: Castle) -> bool {
    for coordinate in [
        castle_move.rook_move.destination,
        castle_move.rook_move.origin,
        castle_move.king_move.destination,
        castle_move.king_move.origin,
    ] {
        if tile::Special::at(coordinate).is_some() {
            eprintln!(
                "Found impossible castle move touching special tiles {:#?}",
                castle_move
            );
            return false;
        }
    }
    true
}
