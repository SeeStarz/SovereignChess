use crate::engine::{
    Coordinate, Gamestate,
    coordinate::Direction,
    faction,
    legal_move::{
        NormalMove,
        calculate::{add_knight_moves_naive, add_linear_moves_naive, add_pawn_moves_naive},
    },
    piece, tile,
};

// BE CAREFUL ABOUT THE ORDER
const DIRECTION_STORE: [Direction; 16] = [
    // Rook
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(0, -1),
    Direction::new(-1, 0),
    // Bishop
    Direction::new(1, 1),
    Direction::new(1, -1),
    Direction::new(-1, 1),
    Direction::new(-1, -1),
    // Knight
    Direction::new(2, 1),
    Direction::new(2, -1),
    Direction::new(1, 2),
    Direction::new(1, -2),
    Direction::new(-1, 2),
    Direction::new(-1, -2),
    Direction::new(-2, 1),
    Direction::new(-2, -1),
];

pub fn get_queen_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..8]
}

pub fn get_rook_directions() -> &'static [Direction] {
    &DIRECTION_STORE[0..4]
}

pub fn get_bishop_directions() -> &'static [Direction] {
    &DIRECTION_STORE[4..8]
}

pub fn get_knight_directions() -> &'static [Direction] {
    &DIRECTION_STORE[8..16]
}

pub fn moves(gamestate: &Gamestate) -> Vec<NormalMove> {
    let mut moves = Vec::new();
    gamestate
        .pieces()
        .filter(|p| {
            gamestate.derived.faction_owners[p.faction as usize]
                == Some(gamestate.c().player_colors[gamestate.c().turn_to_play as usize])
        })
        .for_each(|p| {
            match p.piece_type {
                piece::King | piece::Queen | piece::Rook | piece::Bishop => {
                    add_linear_moves_naive(&mut moves, gamestate, p.into(), p.coordinate);
                }
                piece::Knight => {
                    add_knight_moves_naive(&mut moves, gamestate, p.faction, p.coordinate);
                }
                piece::Pawn => {
                    add_pawn_moves_naive(&mut moves, gamestate, p.faction, p.coordinate);
                }
            };
        });
    moves
}

pub fn try_add_move_check_special_tile_rules(
    gamestate: &Gamestate,
    moves: &mut Vec<NormalMove>,
    move_: NormalMove,
    faction: faction::Color,
) {
    let Some(&special_destination) = tile::Special::at(move_.destination) else {
        moves.push(move_);
        return;
    };

    // Means that we are not trying to occupy special tile colored the same as current faction
    // We are also not trying to occupy special tile where there currently is a piece on the other pair
    if gamestate.is_special_tile_occupiable(special_destination, faction) {
        moves.push(move_);
        return;
    }

    // If the current moved piece is the one on the other pair, it's safe to move there
    if let Some(&special_origin) = tile::Special::at(move_.origin)
        && special_origin.other() == special_destination
    {
        moves.push(move_);
    }
}

// For debug purposes, e.g. allowing a piece to move anywhere
#[allow(dead_code)]
pub fn all_squares(moves: &mut Vec<NormalMove>, origin: Coordinate) {
    for row in 0..16 {
        for col in 0..16 {
            moves.push(NormalMove {
                origin,
                destination: Coordinate::new_unchecked(row, col),
            });
        }
    }
}
