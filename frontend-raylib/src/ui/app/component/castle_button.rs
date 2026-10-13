use crate::{
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
    ui::framework::{input::Event, widget},
    util::Observer,
};
use adapter_core::{Gesture, MenuClick, SessionAction};
use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
};
use shared_model::definition::engine::{FactionId, faction, piece};
use std::{cell::RefCell, rc::Rc};

pub fn build(size: FSize, data: Rc<RefCell<Data>>) -> widget::Builder {
    let mutator = data.clone();
    let observer = Observer::from(data.clone());

    widget::Builder::default()
        .size(size)
        .input(Box::new(move |event, rect| {
            input_handler(event, rect, &mut mutator.borrow_mut())
        }))
        .render(Box::new(move |handle, thread, rect| {
            render_function(handle, thread, rect, &observer.borrow())
        }))
}

fn input_handler(event: Event, rect: FRect, data: &mut Data) -> bool {
    match event {
        Event::MousePressed(position)
            if Rectangle::from(rect).check_collision_point_rec(position) =>
        {
            data.session
                .do_action(SessionAction::Gesture(Gesture::MenuClick(
                    MenuClick::Castle,
                )));
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
) {
    let two_thirds_size = FSize::from(Vec2::from(rect.size) * (2.0 / 3.0));
    let one_third_position =
        FPosition::from(Vec2::from(rect.position) + Vec2::from(rect.size) / 3.0);

    let top_left_rect = FRect {
        position: rect.position,
        size: two_thirds_size,
    };
    let bottom_right_rect = FRect {
        position: one_third_position,
        size: two_thirds_size,
    };

    if data
        .session
        .game_view()
        .intra_game_state
        .valid_castles
        .len()
        > 0
    {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
    } else {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
    }

    handle.draw_composite_pro(
        &data.sprite_manager,
        &PieceSprite {
            piece_type: piece::Rook,
            faction: FactionId::from(faction::White),
            owner: Some(FactionId::from(faction::White)),
        },
        top_left_rect,
        0.0,
        Color::WHITE,
    );

    handle.draw_composite_pro(
        &data.sprite_manager,
        &PieceSprite {
            piece_type: piece::King,
            faction: FactionId::from(faction::White),
            owner: Some(FactionId::from(faction::White)),
        },
        bottom_right_rect,
        0.0,
        Color::WHITE,
    );
}
