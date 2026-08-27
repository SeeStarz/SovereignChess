use crate::engine::{
    Gamestate,
    logic::move_generation::calculate::{knight, linear, pawn},
    model::{Move, piece},
};

pub fn moves(gamestate: &Gamestate) -> Vec<Move> {
    let mut moves = Vec::new();
    gamestate
        .pieces()
        .filter(|p| {
            gamestate.derived.real_faction_owners[p.faction as usize]
                == Some(gamestate.c().player_colors[gamestate.c().turn_to_play as usize])
        })
        .for_each(|p| {
            match p.piece_type {
                // None of these check for checks, it does however check for faction rules
                // TODO: castling and defection
                piece::King | piece::Queen | piece::Rook | piece::Bishop => {
                    linear::add_moves_naive(&mut moves, gamestate, p.into(), p.coordinate);
                }
                piece::Knight => {
                    knight::add_moves_naive(&mut moves, gamestate, p.faction, p.coordinate);
                }
                piece::Pawn => {
                    pawn::add_moves_naive(&mut moves, gamestate, p.faction, p.coordinate);
                }
            };
        });
    // TODO: check for checks
    moves
}
