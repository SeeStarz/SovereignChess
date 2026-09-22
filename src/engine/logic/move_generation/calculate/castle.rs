use crate::engine::{
    GameState, logic,
    {Direction, MoveRich, chess_move::CastleRich, faction, tile},
};

pub fn add_moves_naive(moves: &mut Vec<MoveRich>, game_state: &GameState) {
    for &castle_move in game_state.canonical.remaining_castles.iter() {
        if !check_pieces_allied(game_state, castle_move) {
            continue;
        }

        if !check_path_clear(game_state, castle_move) {
            continue;
        }

        // Because this isn't even supposed to happen and
        // the helper faction check function is unable to process this
        if !check_no_special_tile(castle_move) {
            continue;
        }

        moves.push(MoveRich::Castle(castle_move));
    }
}

fn check_pieces_allied(game_state: &GameState, castle_move: CastleRich) -> bool {
    let Some(king_piece) = game_state.c().board.at(castle_move.king_move.origin) else {
        panic!(
            "Desync with remaining castles. Expected to find king with castle {:#?}",
            castle_move
        )
    };
    let Some(rook_piece) = game_state.c().board.at(castle_move.rook_move.origin) else {
        panic!(
            "Desync with remaining castles. Expected to find king with castle {:#?}",
            castle_move
        )
    };
    if logic::get_allegiance(game_state, king_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    if logic::get_allegiance(game_state, rook_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    true
}

fn check_path_clear(game_state: &GameState, castle_move: CastleRich) -> bool {
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

        if let Some(_piece) = game_state.c().board.at(coordinate) {
            return false;
        }
    }
    true
}

fn check_no_special_tile(castle_move: CastleRich) -> bool {
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
