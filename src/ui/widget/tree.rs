use crate::{
    game::Data,
    geometry::{FPosition, FRect, FSize},
    input,
    render::draw_game,
    ui::{
        Layout, WidgetIntent,
        widget::{ComputedWidget, debug::debug_rect},
    },
    util::Observer,
};
use glam::Vec2;
use std::{cell::RefCell, rc::Rc};

pub fn get(data_mutator: Rc<RefCell<Data>>, data_observer: Observer<Data>) -> ComputedWidget {
    let debug_rect = debug_rect(Layout::Relative(FRect {
        position: FPosition::new(900.0, 100.0),
        size: FSize::new(100.0, 100.0),
    }));
    let board = WidgetIntent {
        children: vec![debug_rect],
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
