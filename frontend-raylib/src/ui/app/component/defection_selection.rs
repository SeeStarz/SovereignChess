use crate::{
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
    ui::{
        app::component::board,
        framework::{
            input::Event,
            widget::{self, prototype::SpecNode},
        },
    },
    util::Observer,
};
use adapter_core::{Gesture, MenuClick};
use engine::{FactionID, faction, logic, piece};
use raylib::{
    RaylibThread,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
};
use std::{cell::RefCell, rc::Rc};
use strum::IntoEnumIterator;

pub fn build(individual_size: FSize, data: Rc<RefCell<Data>>) -> widget::Builder {
    let buttons: Vec<SpecNode> = faction::ColorDefault::iter()
        .map(|f| FactionID::from(f))
        .map(|faction| {
            let mutator = data.clone();
            let observer = Observer::from(data.clone());

            widget::Builder::default()
                .size(individual_size)
                .input(Box::new(move |event, rect| {
                    input_handler(event, rect, &mut mutator.borrow_mut(), faction)
                }))
                .render(Box::new(move |handle, thread, rect| {
                    render_function(handle, thread, rect, &observer.borrow(), faction)
                }))
                .finalize()
        })
        .collect();

    widget::Builder::new_row(true).children(buttons)
}

fn input_handler(event: Event, rect: FRect, data: &mut Data, faction: FactionID) -> bool {
    match event {
        Event::MousePressed(position)
            if Rectangle::from(rect).check_collision_point_rec(position) =>
        {
            board::handle_chess_gesture(Gesture::MenuClick(MenuClick::Defection(faction)), data);
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
    faction: FactionID,
) {
    let real_faction_owners = logic::real_faction_owners(&data.adapter.game_state());

    let hint = &data.cached_hint;
    if hint.valid_defections.contains(&faction) {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
    } else {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
    }

    handle.draw_composite_pro(
        &data.sprite_manager,
        &PieceSprite {
            piece_type: piece::King,
            faction,
            owner: real_faction_owners.get(&faction).map(|_| faction),
        },
        rect,
        0.0,
        Color::WHITE,
    );
}
