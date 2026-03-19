use crate::{
    engine::export::{faction, piece},
    game::Data,
    geometry::{FPosition, FRect, FSize},
    input::{self, Event},
    render::draw_game,
    sprite::{CompositeDraw, PieceSprite},
    ui::{
        Layout, WidgetIntent,
        widget::{ComputedWidget, ignore_input, no_render},
    },
    util::Observer,
};
use glam::Vec2;
use raylib::{color::Color, math::Rectangle, prelude::RaylibDraw};
use std::{cell::RefCell, rc::Rc};

pub fn get(data_mutator: Rc<RefCell<Data>>, data_observer: Observer<Data>) -> ComputedWidget {
    let mut last_selected = None;
    for piece_type in [
        piece::King,
        piece::Queen,
        piece::Rook,
        piece::Bishop,
        piece::Knight,
    ] {
        let promotion_selector = WidgetIntent {
            children: last_selected.map_or(vec![], |s| vec![s]),
            layout: Layout::Relative(FRect {
                position: FPosition::new(64.0, 0.0),
                size: FSize::new(64.0, 64.0),
            }),
            input_handler: Box::new({
                let d = data_mutator.clone();
                move |event, rect| match event {
                    Event::MousePressed(position)
                        if Rectangle::from(rect).check_collision_point_rec(position) =>
                    {
                        let mut d = d.borrow_mut();
                        if let Some(selected_piece_type) = d.selected_piece_type {
                            if selected_piece_type == piece_type {
                                d.selected_piece_type = None;
                            } else {
                                d.selected_piece_type = Some(piece_type);
                            }
                        } else {
                            d.selected_piece_type = Some(piece_type);
                        }
                        true
                    }
                    _ => false,
                }
            }),
            render_function: Box::new({
                let d = data_observer.clone();
                move |handle, _thread, rect| {
                    if d.borrow()
                        .selected_piece_type
                        .is_none_or(|s| s != piece_type)
                    {
                        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
                    } else {
                        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
                    }
                    handle.draw_composite_pro(
                        &d.borrow().sprite_manager,
                        &PieceSprite {
                            piece_type,
                            faction: faction::White,
                            owner: Some(faction::White),
                        },
                        rect,
                        0.0,
                        Color::WHITE,
                    );
                }
            }),
        };

        last_selected = Some(promotion_selector);
    }

    let offsetter = WidgetIntent {
        children: vec![last_selected.unwrap()],
        layout: Layout::Relative(FRect {
            position: FPosition::new(600.0, 200.0),
            size: FSize::default(),
        }),
        input_handler: Box::new(ignore_input),
        render_function: Box::new(no_render),
    };

    let board = WidgetIntent {
        children: vec![offsetter],
        layout: Layout::Relative(FRect {
            position: FPosition::default(),
            size: FSize::from(Vec2::new(32.0 * 16.0, 32.0 * 16.0)),
        }),
        input_handler: Box::new(move |event, rect| {
            input::handle_board_input(event, rect, &mut data_mutator.borrow_mut())
        }),
        render_function: Box::new({
            let d = data_observer.clone();
            move |handle, thread, rect| {
                draw_game(handle, thread, rect, &d.borrow());
            }
        }),
    };
    board.compute(FRect {
        position: FPosition::from(Vec2::new(32.0, 32.0)),
        size: FSize::default(),
    })
}
