use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    math::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    engine::export::{
        Coordinate, Move,
        legal_move::{NormalMove, Promotion, RegimeChangePromotion},
        piece, tile,
    },
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
    let Some(coordinate1) = Coordinate::new(position.y as i32, position.x as i32) else {
        return false;
    };

    if let Some(coordinate2) = data.selected_square {
        let attempted_move = if let Some(piece_type) = data.selected_piece_type {
            if piece_type == piece::King {
                Move::RegimeChangePromotion(RegimeChangePromotion {
                    normal_move: NormalMove {
                        origin: coordinate2,
                        destination: coordinate1,
                    },
                })
            } else {
                Move::Promotion(Promotion {
                    normal_move: NormalMove {
                        origin: coordinate2,
                        destination: coordinate1,
                    },
                    piece_type,
                })
            }
        } else {
            Move::NormalMove(NormalMove {
                origin: coordinate2,
                destination: coordinate1,
            })
        };

        if data.legal_moves.iter().any(|&x| x == attempted_move) {
            data.gamestate = data.gamestate.apply_move(attempted_move);
            data.legal_moves = data.gamestate.moves();
        }
        data.selected_square = None;
    } else {
        if data
            .gamestate
            .pieces()
            .find(|p| p.coordinate == coordinate1)
            .is_some()
        {
            data.selected_square = Some(coordinate1);
        }
    }

    true
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

    for move_ in data.legal_moves.iter().filter_map(|&mv| {
        if let Move::NormalMove(nmv) = mv {
            if nmv.origin == selected_square && data.selected_piece_type.is_none() {
                Some(nmv)
            } else {
                None
            }
        } else if let Move::Promotion(promotion_move) = mv {
            if promotion_move.normal_move.origin == selected_square
                && data.selected_piece_type.is_some()
            {
                Some(promotion_move.normal_move)
            } else {
                None
            }
        } else {
            None
        }
    }) {
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
