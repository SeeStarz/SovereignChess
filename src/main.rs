mod engine;
mod sprite;

use crate::engine::export::{Coordinate, Gamestate, Move, faction, piece, tile};
use raylib::prelude::*;

struct Data {
    gamestate: Gamestate,
    legal_moves: Vec<Move>,
    selected_square: Option<Coordinate>,
    sprite_manager: sprite::Manager,
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .resizable()
        .size(1080, 720)
        .title("Sovereign Chess")
        .vsync()
        .build();

    let mut data = {
        let gamestate = Gamestate::new();
        Data {
            gamestate,
            legal_moves: gamestate.get_moves(),
            selected_square: None,
            sprite_manager: sprite::Manager::new(&mut raylib_handle, &thread),
        }
    };

    while !raylib_handle.window_should_close() {
        if raylib_handle.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let row = (raylib_handle.get_mouse_y() / 32) - 1;
            let col = (raylib_handle.get_mouse_x() / 32) - 1;
            if let Some(coordinate1) = Coordinate::new(row, col) {
                if let Some(coordinate2) = data.selected_square {
                    let attempted_move = Move {
                        origin: coordinate2,
                        destination: coordinate1,
                    };

                    if data
                        .legal_moves
                        .iter()
                        .find(|&&x| x == attempted_move)
                        .is_some()
                    {
                        data.gamestate = data.gamestate.apply_move(attempted_move);
                        data.legal_moves = data.gamestate.get_moves();
                    }
                    data.selected_square = None;
                } else if data
                    .gamestate
                    .pieces()
                    .find(|p| p.coordinate == coordinate1)
                    .is_some()
                {
                    data.selected_square = Some(coordinate1);
                }
            }
        }

        //// Below is the preferred way of doing things, but is still broken (double END_DRAWING) as per version 5.5.1
        // raylib_handle.draw(&thread, |handle| draw(handle, &thread, &data));
        //// Working version
        {
            let draw_handle = raylib_handle.begin_drawing(&thread);
            draw(draw_handle, &thread, &data);
        }
    }
}

fn draw(mut handle: RaylibDrawHandle, thread: &RaylibThread, data: &Data) {
    handle.clear_background(Color::BLACK);

    draw_board(&mut handle, thread, data);
    draw_pieces(&mut handle, thread, data);
    draw_legal_moves(&mut handle, thread, data);
}

fn draw_board(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _data: &Data) {
    for r in 0..16 {
        for c in 0..16 {
            let coordinate = Coordinate::new_unchecked(r, c);
            let color = if let Some(special) = tile::Special::at(&coordinate) {
                special.faction.to_color()
            } else {
                if (r + c) % 2 == 0 {
                    Color::BROWN
                } else {
                    Color::LIGHTGRAY
                }
            };

            let size = 32;
            handle.draw_rectangle(size + c * size, size + r * size, size, size, color);
        }
    }
}

trait ToSprite {
    fn to_sprite(self) -> sprite::Prototype;
}

impl ToSprite for piece::Type {
    fn to_sprite(self) -> sprite::Prototype {
        use piece::Type::*;
        use sprite::Prototype::*;
        match self {
            King => KingOwnerMask,
            Queen => QueenOwnerMask,
            Rook => RookOwnerMask,
            Bishop => BishopOwnerMask,
            Knight => KnightOwnerMask,
            Pawn => PawnOwnerMask,
        }
    }
}

trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for faction::Color {
    fn to_color(self) -> Color {
        use faction::Color::*;
        match self {
            White => Color::WHITE,
            Pink => Color::PINK,
            Slate => Color::SLATEGRAY,
            Red => Color::RED,
            Orange => Color::ORANGE,
            Yellow => Color::YELLOW,
            Green => Color::GREEN,
            Cyan => Color::CYAN,
            Navy => Color::NAVY,
            Ash => Color::GRAY,
            Violet => Color::VIOLET,
            Black => Color::BLACK,
        }
    }
}

fn draw_pieces(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, data: &Data) {
    let size = 32;
    for piece in data.gamestate.pieces() {
        let texture = data.sprite_manager.get(piece.piece_type.to_sprite()).1;
        handle.draw_texture(
            texture,
            piece.coordinate.col as i32 * size + size,
            piece.coordinate.row as i32 * size + size,
            piece.faction.to_color(),
        );
    }
}

fn draw_legal_moves(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, data: &Data) {
    let Some(selected_square) = data.selected_square else {
        return;
    };

    let size = 32;
    for move_ in data
        .legal_moves
        .iter()
        .filter(|&&mv| mv.origin == selected_square)
    {
        handle.draw_circle(
            (move_.destination.col as i32 + 1) * size + size / 2,
            (move_.destination.row as i32 + 1) * size + size / 2,
            size as f32 / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}
