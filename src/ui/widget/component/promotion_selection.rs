use std::{cell::RefCell, rc::Rc};

use crate::{
    engine::export::{faction, piece},
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
    ui::{
        input::Event,
        widget::{WidgetSpecNode, builder::WidgetBuilder},
    },
    util::Observer,
};
use raylib::{
    RaylibThread,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
};

pub fn make_promotion_selection_buttons(
    individual_size: FSize,
    data: Rc<RefCell<Data>>,
) -> WidgetBuilder {
    let buttons: Vec<WidgetSpecNode> = [
        piece::King,
        piece::Queen,
        piece::Rook,
        piece::Bishop,
        piece::Knight,
    ]
    .into_iter()
    .map(|piece_type| {
        let mutator = data.clone();
        let observer = Observer::from(data.clone());

        WidgetBuilder::default()
            .size(individual_size)
            .input(Box::new(move |event, rect| {
                input_handler(event, rect, &mut mutator.borrow_mut(), piece_type)
            }))
            .render(Box::new(move |handle, thread, rect| {
                render_function(handle, thread, rect, &observer.borrow(), piece_type)
            }))
            .finalize()
    })
    .collect();

    WidgetBuilder::new_row(true).children(buttons)
}

fn input_handler(event: Event, rect: FRect, data: &mut Data, piece_type: piece::Type) -> bool {
    match event {
        Event::MousePressed(position)
            if Rectangle::from(rect).check_collision_point_rec(position) =>
        {
            if let Some(selected_piece_type) = data.selected_piece_type {
                if selected_piece_type == piece_type {
                    data.selected_piece_type = None;
                } else {
                    data.selected_piece_type = Some(piece_type);
                }
            } else {
                data.selected_piece_type = Some(piece_type);
            }
            true
        }
        _ => false,
    }
}

pub fn render_function(
    handle: &mut RaylibDrawHandle,
    _thread: &RaylibThread,
    rect: FRect,
    data: &Data,
    piece_type: piece::Type,
) {
    if data.selected_piece_type.is_none_or(|s| s != piece_type) {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
    } else {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
    }
    handle.draw_composite_pro(
        &data.sprite_manager,
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
