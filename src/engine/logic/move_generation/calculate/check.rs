use crate::engine::{
    Gamestate, logic,
    model::{Direction, Move, chess_move::Castle},
};

pub fn filter_checks(gamestate: &Gamestate, moves: &mut Vec<Move>) {
    moves.retain_mut(|&mut m| {
        if let Move::Castle(castle_move) = m {
            is_castle_safe(gamestate, castle_move)
        } else {
            let do_move_gamestate = gamestate.apply_move(m);
            is_enemy_king_safe(&do_move_gamestate)
        }
    });
}

fn is_castle_safe(gamestate: &Gamestate, castle_move: Castle) -> bool {
    let king_offset = Direction::from_coordinate_pair(
        castle_move.king_move.origin,
        castle_move.king_move.destination,
    );
    let move_distance = king_offset.manhattan_distance();
    let king_direction = Direction::new(
        king_offset.row / move_distance as i32,
        king_offset.col / move_distance as i32,
    );

    for distance in 0..=move_distance {
        let Some(coordinate) = castle_move
            .king_move
            .origin
            .offset(king_direction * distance as i32)
        else {
            panic!("Castling out of bounds")
        };

        let Some(piece) = gamestate.c().board.at(coordinate) else {
            panic!("King not found at {:?}", coordinate);
        };

        let mut temporary_gamestate = gamestate.clone();
        temporary_gamestate
            .canonical
            .board
            .set_at(castle_move.king_move.origin, None);
        temporary_gamestate
            .canonical
            .board
            .set_at(coordinate, Some(piece));
        temporary_gamestate.canonical.turn_to_play =
            temporary_gamestate.canonical.turn_to_play.other();
        if !is_enemy_king_safe(&temporary_gamestate) {
            return false;
        }
    }

    is_enemy_king_safe(&gamestate.apply_move(Move::Castle(castle_move)))
}

fn is_enemy_king_safe(gamestate: &Gamestate) -> bool {
    logic::move_generation::calculate::naive_moves(gamestate)
        .iter()
        .all(|&m| {
            let response_gamestate = gamestate.apply_move(m);
            logic::board::find_current_player_king(&response_gamestate).is_some()
        })
}
