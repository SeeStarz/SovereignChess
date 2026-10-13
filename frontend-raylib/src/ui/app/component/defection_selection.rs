use crate::{
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
    ui::framework::{
        input::Event,
        widget::{self, prototype::SpecNode},
    },
    util::Observer,
};
use adapter_core::{Gesture, MenuClick, SessionAction};
use raylib::{
    RaylibThread,
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
};
use shared_model::definition::engine::{FactionId, piece};
use std::{cell::RefCell, rc::Rc};

pub fn build(individual_size: FSize, data: Rc<RefCell<Data>>) -> widget::Builder {
    let mut factions = data.borrow().session.game_view().game_state.factions;
    factions.sort_by(|a, b| a.0.cmp(&b.0));

    let buttons: Vec<SpecNode> = factions
        .iter()
        .map(|&faction| {
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

fn input_handler(event: Event, rect: FRect, data: &mut Data, faction: FactionId) -> bool {
    match event {
        Event::MousePressed(position)
            if Rectangle::from(rect).check_collision_point_rec(position) =>
        {
            data.session
                .do_action(SessionAction::Gesture(Gesture::MenuClick(
                    MenuClick::Defection(faction),
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
    faction: FactionId,
) {
    if data
        .session
        .game_view()
        .intra_game_state
        .valid_defections
        .contains(&faction)
    {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::BLUE);
    } else {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::GRAY);
    }

    handle.draw_composite_pro(
        &data.sprite_manager,
        &PieceSprite {
            piece_type: piece::King,
            faction,
            owner: data
                .session
                .game_view()
                .game_state
                .owned_factions
                .get(&faction)
                .map(|_| faction),
        },
        rect,
        0.0,
        Color::WHITE,
    );
}
