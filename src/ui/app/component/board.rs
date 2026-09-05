use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    math::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    adapter::{Adapter, Click},
    engine::export::{Coordinate, tile},
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
    let Some(click_coordinate) = Coordinate::new(position.y as i32, position.x as i32) else {
        return false;
    };

    data.adapter.reset();

    let has_origin = if let Some(first_coordinate) = data.selected_square {
        if !data.adapter.click(Click::BoardClick(first_coordinate)) {
            data.selected_square = None;
            return true;
        }
        true
    } else {
        false
    };

    if !data.adapter.click(Click::BoardClick(click_coordinate)) {
        data.selected_square = None;
        return true;
    } else if !has_origin {
        data.selected_square = Some(click_coordinate);
        assert!(data.adapter.data().gamestate_change.is_none());
        return true;
    }

    if let Some(piece_type) = data.selected_piece_type {
        if !data.adapter.click(Click::PromotionClick(piece_type)) {
            data.selected_square = None;
            return true;
        }
    }

    if let Some(gamestate_change) = data.adapter.data().gamestate_change {
        data.gamestate = gamestate_change.gamestate;
        data.adapter = Adapter::new(gamestate_change.gamestate);
        true
    } else {
        // TODO: this is likely when it should be a promotion but no piece is selected and vice versa
        data.selected_square = None;
        true
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
    for click in data.adapter.data().valid_clicks {
        let position = match click {
            Click::BoardClick(destination) => Some(
                Vec2::new(
                    destination.col() as f32 + 0.5,
                    destination.row() as f32 + 0.5,
                ) * Vec2::from(tile_rect.size)
                    + Vec2::from(tile_rect.position),
            ),
            // TODO:
            _ => None,
        };
        let Some(position) = position else {
            continue;
        };

        handle.draw_ellipse(
            position.x as i32,
            position.y as i32,
            tile_rect.size.width / 3.0,
            tile_rect.size.height / 3.0,
            Color::BLUE.alpha(0.25),
        );
    }
}
