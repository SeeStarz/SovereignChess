use crate::{
    Coordinate, GameState, MoveRich, MoveSimple, Vec2,
    chess_move::{CastleRich, CastleSimple},
    logic,
};

pub fn filter_checks(game_state: &GameState, moves: &mut Vec<MoveRich>) {
    moves.retain_mut(|&mut m| {
        if let MoveRich::Castle(castle_move) = m {
            is_castle_safe(game_state, castle_move)
        } else {
            let do_move_game_state = game_state.apply_move(MoveSimple::from(m));
            is_enemy_king_safe(&do_move_game_state)
        }
    });
}

fn is_castle_safe(game_state: &GameState, castle_move: CastleRich) -> bool {
    let king_offset = Vec2::from_coordinate_pair(
        castle_move.king_move.origin,
        castle_move.king_move.destination,
    );
    let move_distance = king_offset.manhattan_distance();
    let king_direction = Vec2::new(
        king_offset.row / move_distance as i32,
        king_offset.col / move_distance as i32,
    );

    for distance in 0..=move_distance {
        let coordinate = castle_move
            .king_move
            .origin
            .offset(king_direction * distance as i32);

        if !game_state.board.is_coordinate_valid(coordinate) {
            panic!("Castling out of bounds")
        };

        let mut temporary_game_state = game_state.clone();
        temporary_game_state.turn_manager = temporary_game_state.turn_manager.next_turn();
        while temporary_game_state.turn_manager.current_player()
            != game_state.turn_manager.current_player()
        {
            if is_coordinate_attackable(&temporary_game_state, coordinate) {
                return false;
            }
        }
    }

    is_enemy_king_safe(&game_state.apply_move(MoveSimple::Castle(CastleSimple::from(castle_move))))
}

fn is_enemy_king_safe(game_state: &GameState) -> bool {
    logic::move_generation::calculate::naive_moves(game_state)
        .iter()
        .all(|&m| {
            let response_game_state = game_state.apply_move(MoveSimple::from(m));
            logic::find_current_player_king(&response_game_state).is_some()
        })
}

fn is_coordinate_attackable(game_state: &GameState, coordinate: Coordinate) -> bool {
    logic::move_generation::calculate::naive_moves(game_state)
        .iter()
        .any(|&m| move_attacked_square(m).is_some_and(|c| c == coordinate))
}

fn move_attacked_square(chess_move: MoveRich) -> Option<Coordinate> {
    match chess_move {
        MoveRich::NormalMove(normal_move) => Some(normal_move.destination),
        MoveRich::Castle(_castle_move) => None,
        MoveRich::Defection(defection_move) => {
            Some(defection_move.destination.unwrap_or(defection_move.origin))
        }
        MoveRich::RegimeChangePromotion(promotion_move) => {
            Some(promotion_move.pawn_move.destination)
        }
        MoveRich::Promotion(promotion_move) => Some(promotion_move.normal_move.destination),
    }
}
