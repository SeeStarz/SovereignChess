mod engine;
mod sprite;

use crate::engine::export::{Coordinate, Gamestate, faction, piece, tile};
use raylib::prelude::*;

struct Data {
    gamestate: Gamestate,
    sprite_manager: sprite::Manager,
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .resizable()
        .size(1080, 720)
        .title("Sovereign Chess")
        .vsync()
        .build();

    let data = Data {
        gamestate: Gamestate::new(),
        sprite_manager: sprite::Manager::new(&mut raylib_handle, &thread),
    };

    while !raylib_handle.window_should_close() {
        raylib_handle.draw(&thread, |handle| draw(handle, &thread, &data));
    }
}

fn draw(mut handle: RaylibDrawHandle, thread: &RaylibThread, data: &Data) {
    handle.clear_background(Color::BLACK);

    draw_board(&mut handle, thread, data);
    draw_pieces(&mut handle, thread, data);
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
        if piece.piece_type == piece::King {
            println!("{:#?}", piece);
        }
        let texture = data.sprite_manager.get(piece.piece_type.to_sprite()).1;
        handle.draw_texture(
            texture,
            piece.coordinate.col as i32 * size + size,
            piece.coordinate.row as i32 * size + size,
            piece.faction.to_color(),
        );
    }
}
