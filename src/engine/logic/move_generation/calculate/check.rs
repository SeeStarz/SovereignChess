use crate::engine::{
    GameState, logic,
    model::{Direction, Move, chess_move::Castle},
};

pub fn filter_checks(game_state: &GameState, moves: &mut Vec<Move>) {
    moves.retain_mut(|&mut m| {
        if let Move::Castle(castle_move) = m {
            is_castle_safe(game_state, castle_move)
        } else {
            let do_move_game_state = game_state.apply_move(m);
            is_enemy_king_safe(&do_move_game_state)
        }
    });
}

fn is_castle_safe(game_state: &GameState, castle_move: Castle) -> bool {
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

        let Some(piece) = game_state.c().board.at(coordinate) else {
            panic!("King not found at {:?}", coordinate);
        };

        let mut temporary_game_state = game_state.clone();
        temporary_game_state
            .canonical
            .board
            .set_at(castle_move.king_move.origin, None);
        temporary_game_state
            .canonical
            .board
            .set_at(coordinate, Some(piece));
        temporary_game_state.canonical.turn_to_play =
            temporary_game_state.canonical.turn_to_play.other();
        if !is_enemy_king_safe(&temporary_game_state) {
            return false;
        }
    }

    is_enemy_king_safe(&game_state.apply_move(Move::Castle(castle_move)))
}

fn is_enemy_king_safe(game_state: &GameState) -> bool {
    logic::move_generation::calculate::naive_moves(game_state)
        .iter()
        .all(|&m| {
            let response_game_state = game_state.apply_move(m);
            logic::board::find_current_player_king(&response_game_state).is_some()
        })
}
