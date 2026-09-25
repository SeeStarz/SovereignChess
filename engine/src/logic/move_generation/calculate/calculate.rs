use crate::{
    GameState, MoveRich,
    logic::{
        self, find_current_player_king,
        move_generation::calculate::{castle, check, defection, knight, linear, pawn},
    },
    piece,
};

pub fn naive_moves(game_state: &GameState) -> Vec<MoveRich> {
    let current_player_faction = logic::current_player_faction(game_state);
    let mut moves = Vec::new();
    game_state
        .pieces()
        .filter(|p| p.owner == Some(current_player_faction))
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

    if let Some(king_piece) = find_current_player_king(game_state) {
        defection::add_moves_naive(
            &mut moves,
            game_state,
            current_player_faction,
            king_piece.coordinate,
        );
    }

    moves
}

pub fn moves(game_state: &GameState) -> Vec<MoveRich> {
    let moves = naive_moves(game_state);
    // TODO: Optimize and re-enable filter_checks
    check::filter_checks(game_state, &mut vec![]);
    moves
}
