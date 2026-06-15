use crate::{
    game::Data,
    geometry::FSize,
    ui::{
        app::component,
        framework::widget::{self, prototype::ComputedNode},
    },
};
use glam::Vec2;
use std::{cell::RefCell, rc::Rc};

pub fn generate(data: Rc<RefCell<Data>>) -> ComputedNode {
    let board = component::board(
        FSize::from(Vec2::new(32.0 * 16.0, 32.0 * 16.0)),
        data.clone(),
    )
    .finalize();

    let spacer1 = widget::Builder::default()
        .size(FSize::new(0.0, 32.0))
        .finalize();

    let promotion_selection_buttons =
        component::promotion_selection(FSize::new(32.0, 32.0), data.clone()).finalize();

    let spacer2 = widget::Builder::default()
        .size(FSize::new(0.0, 32.0))
        .finalize();

    let debug_toggle_button = component::debug::toggle_button(FSize::new(100.0, 100.0)).finalize();

    let root = widget::Builder::new_col(true)
        .children(vec![
            board,
            spacer1,
            promotion_selection_buttons,
            spacer2,
            debug_toggle_button,
        ])
        .finalize();

    root.compute()
}
