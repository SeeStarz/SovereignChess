use raylib::{
    RaylibThread,
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    Data, FPosition, FRect, FSize,
    engine::export::{Coordinate, faction, tile},
    sprite::{CompositeDraw, PieceSprite},
    ui::widget::ComputedWidget,
};

pub trait ToColor {
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
            Black => Color::BLACK.brightness(0.2),
        }
    }
}

pub fn draw(mut handle: RaylibDrawHandle, thread: &RaylibThread, widget_tree: &ComputedWidget) {
    handle.clear_background(Color::BLACK);
    widget_tree
        .iter()
        .for_each(|w| (w.render_function)(&mut handle, thread, &w.rect));
}

pub fn draw_game(handle: &mut RaylibDrawHandle, thread: &RaylibThread, data: &Data) {
    draw_board(handle, thread, data);
    draw_pieces(handle, thread, data);
    draw_legal_moves(handle, thread, data);
}

fn draw_board(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _data: &Data) {
    for r in 0..16 {
        for c in 0..16 {
            let coordinate = Coordinate::new_unchecked(r, c);
            let color = if let Some(special) = tile::Special::at(coordinate) {
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

fn draw_pieces(handle: &mut RaylibDrawHandle, _thread: &RaylibThread, data: &Data) {
    let size = 32.0;
    for piece in data.gamestate.pieces() {
        let sprite = PieceSprite {
            piece_type: piece.piece_type,
            faction: piece.faction,
            owner: piece.owner,
        };

        let dest = FRect {
            position: FPosition {
                x: piece.coordinate.col() as f32 * size + size,
                y: piece.coordinate.row() as f32 * size + size,
            },
            size: FSize {
                width: size,
                height: size,
            },
        };

        handle.draw_composite_pro(&data.sprite_manager, &sprite, dest, 0.0, Color::WHITE);
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
            (move_.destination.col() as i32 + 1) * size + size / 2,
            (move_.destination.row() as i32 + 1) * size + size / 2,
            size as f32 / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}
