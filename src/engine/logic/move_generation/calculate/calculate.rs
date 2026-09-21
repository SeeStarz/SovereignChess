use crate::engine::{
    GameState,
    logic::{
        self,
        board::find_current_player_king_assert,
        move_generation::calculate::{castle, defection, knight, linear, pawn},
    },
    model::{Move, piece},
};

pub fn naive_moves(game_state: &GameState) -> Vec<Move> {
    let mut moves = Vec::new();
    game_state
        .pieces()
        .filter(|p| {
            game_state.derived.real_faction_owners[p.faction as usize]
                == Some(game_state.c().player_colors[game_state.c().turn_to_play as usize])
        })
        .for_each(|p| {
            match p.piece_type {
                // None of these check for checks, it does however check for faction rules
                piece::King | piece::Queen | piece::Rook | piece::Bishop => {
                    linear::add_moves_naive(&mut moves, game_state, p.into(), p.coordinate);
                }
                piece::Knight => {
                    knight::add_moves_naive(&mut moves, game_state, p.faction, p.coordinate);
                }
                piece::Pawn => {
                    pawn::add_moves_naive(&mut moves, game_state, p.faction, p.coordinate);
                }
            };
        });
    castle::add_moves_naive(&mut moves, game_state);
    defection::add_moves_naive(
        &mut moves,
        game_state,
        game_state.c().player_colors[game_state.c().turn_to_play as usize],
        find_current_player_king_assert(game_state).coordinate,
    );

    // TODO: check for checks
    moves
}

pub fn moves(game_state: &GameState) -> Vec<Move> {
    let mut moves = naive_moves(game_state);
    logic::move_generation::calculate::check::filter_checks(game_state, &mut moves);
    moves
}
