use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    engine::export::{Coordinate, faction, tile},
    game::Data,
    geometry::{FPosition, FRect, FSize},
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
        .for_each(|w| (w.render_function)(&mut handle, thread, w.rect));
}

pub fn draw_game(handle: &mut RaylibDrawHandle, thread: &RaylibThread, rect: FRect, data: &Data) {
    let tile_rect = FRect {
        position: rect.position,
        size: FSize::from(Vec2::from(rect.size) / 16.0),
    };
    draw_board(handle, thread, tile_rect, data);
    draw_pieces(handle, thread, tile_rect, data);
    draw_legal_moves(handle, thread, tile_rect, data);
}

fn draw_board(
    handle: &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    tile_rect: FRect,
    _data: &Data,
) {
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

            let position = Vec2::new(c as f32, r as f32) * Vec2::from(tile_rect.size)
                + Vec2::from(tile_rect.position);
            handle.draw_rectangle_pro(
                FRect {
                    position: FPosition::from(position),
                    size: tile_rect.size,
                },
                FPosition::default(),
                0.0,
                color,
            );
        }
    }
}

fn draw_pieces(
    handle: &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    tile_rect: FRect,
    data: &Data,
) {
    for piece in data.gamestate.pieces() {
        let sprite = PieceSprite {
            piece_type: piece.piece_type,
            faction: piece.faction,
            owner: piece.owner,
        };

        let position = Vec2::new(piece.coordinate.col() as f32, piece.coordinate.row() as f32)
            * Vec2::from(tile_rect.size)
            + Vec2::from(tile_rect.position);
        let dest = FRect {
            position: FPosition::from(position),
            size: tile_rect.size,
        };

        handle.draw_composite_pro(&data.sprite_manager, &sprite, dest, 0.0, Color::WHITE);
    }
}

fn draw_legal_moves(
    handle: &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    tile_rect: FRect,
    data: &Data,
) {
    let Some(selected_square) = data.selected_square else {
        return;
    };

    for &move_ in data
        .legal_moves
        .iter()
        .filter(|&&mv| mv.origin == selected_square)
    {
        let position = Vec2::new(
            move_.destination.col() as f32 + 0.5,
            move_.destination.row() as f32 + 0.5,
        ) * Vec2::from(tile_rect.size)
            + Vec2::from(tile_rect.position);

        handle.draw_ellipse(
            position.x as i32,
            position.y as i32,
            tile_rect.size.width / 3.0,
            tile_rect.size.height / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}
