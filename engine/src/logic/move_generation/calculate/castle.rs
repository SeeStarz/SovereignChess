use crate::{
    Board, CastleSource, GameState, MoveRich, Vec2,
    chess_move::{CastleRich, NormalMove},
    faction, logic,
};

pub fn add_moves_naive(moves: &mut Vec<MoveRich>, game_state: &GameState) {
    for &castle_source in game_state.remaining_castles.iter() {
        if !check_pieces_allied(game_state, castle_source) {
            continue;
        }

        if !check_path_clear(game_state, castle_source) {
            continue;
        }

        add_castle_source(moves, &game_state.board, castle_source);
    }
}

fn check_pieces_allied(game_state: &GameState, castle_source: CastleSource) -> bool {
    let Some(king_piece) = game_state
        .board
        .at(castle_source.king_coordinate)
        .expect("CastleSource out of bounds")
        .0
    else {
        panic!(
            "Desync with remaining castles. Expected to find king at {:?}",
            castle_source.king_coordinate
        )
    };
    let Some(rook_piece) = game_state
        .board
        .at(castle_source.rook_coordinate)
        .expect("CastleSource out of bounds")
        .0
    else {
        panic!(
            "Desync with remaining castles. Expected to find rook at {:?}",
            castle_source.rook_coordinate
        )
    };
    if logic::allegiance(game_state, king_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    if logic::allegiance(game_state, rook_piece.faction) != faction::Allegiance::Ally {
        return false;
    }
    true
}

fn check_path_clear(game_state: &GameState, castle_source: CastleSource) -> bool {
    let sweep_vector =
        Vec2::from_coordinate_pair(castle_source.rook_coordinate, castle_source.king_coordinate);
    let sweep_distance = sweep_vector.manhattan_distance() - 1;
    let sweep_direction = Vec2::new(
        sweep_vector.row / sweep_distance as i32,
        sweep_vector.col / sweep_distance as i32,
    );

    for distance in 1..=sweep_distance {
        let coordinate = castle_source
            .rook_coordinate
            .offset(sweep_direction * distance as i32);

        if let Some(_piece) = game_state
            .board
            .at(coordinate)
            .expect("Castling out of bounds")
            .0
        {
            return false;
        }
    }
    true
}

fn add_castle_source(moves: &mut Vec<MoveRich>, board: &Board, castle_source: CastleSource) {
    let sweep_vector =
        Vec2::from_coordinate_pair(castle_source.king_coordinate, castle_source.rook_coordinate);
    let sweep_distance = sweep_vector.manhattan_distance() - 1;
    let sweep_direction = Vec2::new(
        sweep_vector.row / sweep_distance as i32,
        sweep_vector.col / sweep_distance as i32,
    );

    for distance in 1..=sweep_distance {
        let king_end_coordinate = castle_source
            .king_coordinate
            .offset(sweep_direction * distance as i32);
        if !board.is_coordinate_valid(king_end_coordinate) {
            panic!("Castling out of bounds")
        };

        let rook_end_coordinate = castle_source
            .king_coordinate
            .offset(sweep_direction * (distance - 1) as i32);
        if !board.is_coordinate_valid(rook_end_coordinate) {
            panic!("Castling out of bounds")
        };

        moves.push(MoveRich::Castle(CastleRich {
            king_move: NormalMove {
                origin: castle_source.king_coordinate,
                destination: king_end_coordinate,
            },
            rook_move: NormalMove {
                origin: castle_source.rook_coordinate,
                destination: rook_end_coordinate,
            },
        }));
    }
}
