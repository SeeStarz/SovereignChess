use std::{cell::RefCell, rc::Rc};

use crate::{
    game::Data,
    geometry::{FPosition, FRect, FSize},
    render::ToColor,
    sprite::{CompositeDraw, PieceSprite},
    ui::framework::{
        input::Event,
        widget::{
            self,
            prototype::{InputHandler, RenderFunction},
        },
    },
    util::Observer,
};
use adapter_core::{Adapter, BoardGesture, Gesture};
use engine::{Coordinate, tile};
use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    math::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub fn build(size: FSize, data: Rc<RefCell<Data>>) -> widget::Builder {
    let observer = Observer::from(data.clone());

    let input_handler: InputHandler =
        Box::new(move |event, rect| handle_input(event, rect, &mut data.borrow_mut()));

    let render_function: RenderFunction = Box::new(move |handle, thread, rect| {
        draw_game(handle, thread, rect, &observer.borrow());
    });

    widget::Builder::default()
        .size(size)
        .input(input_handler)
        .render(render_function)
}

fn handle_input(event: Event, rect: FRect, data: &mut Data) -> bool {
    let Event::MousePressed(click_position) = event else {
        return false;
    };
    if !Rectangle::from(rect).check_collision_point_rec(click_position) {
        return false;
    }

    let tile_size = Vec2::from(rect.size) / 16.0;
    let position = (Vec2::from(click_position) - Vec2::from(rect.position)) / tile_size;
    let click_coordinate = Coordinate::new(position.y as i32, position.x as i32);

    handle_chess_gesture(Gesture::Board(BoardGesture::Click(click_coordinate)), data);
    true
}

pub fn handle_chess_gesture(gesture: Gesture, data: &mut Data) {
    data.adapter.apply(gesture);
    let hint = data.adapter.hint();
    if let Some(game_state_change) = hint.game_state_change.clone() {
        data.adapter = Adapter::new(game_state_change.updated_game_state);
        data.cached_hint = data.adapter.hint();
    } else {
        data.cached_hint = hint;
    }
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
            let coordinate = Coordinate::new(r, c);
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
    for piece in data.adapter.game_state().pieces() {
        let sprite = PieceSprite {
            piece_type: piece.piece_type,
            faction: piece.faction,
            owner: piece.owner,
        };

        let position = Vec2::new(piece.coordinate.col as f32, piece.coordinate.row as f32)
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
    let hint = &data.cached_hint;

    if let Some(piece) = hint.grabbed_piece {
        let position = coordinate_to_centered_position(piece.coordinate, tile_rect);
        handle.draw_ellipse(
            position.x as i32,
            position.y as i32,
            tile_rect.size.width / 3.0,
            tile_rect.size.height / 3.0,
            Color::GREEN.alpha(0.25),
        );
    }

    for &destination in hint.valid_destinations.iter() {
        let position = coordinate_to_centered_position(destination, tile_rect);
        handle.draw_ellipse(
            position.x as i32,
            position.y as i32,
            tile_rect.size.width / 3.0,
            tile_rect.size.height / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}

fn coordinate_to_centered_position(coordinate: Coordinate, tile_rect: FRect) -> Vec2 {
    Vec2::new(coordinate.col as f32 + 0.5, coordinate.row as f32 + 0.5) * Vec2::from(tile_rect.size)
        + Vec2::from(tile_rect.position)
}
