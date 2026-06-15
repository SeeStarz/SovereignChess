use crate::{
    game::Data,
    geometry::FSize,
    ui::widget::{
        ComputedWidgetNode,
        builder::WidgetBuilder,
        component::{self, promotion_selection::make_promotion_selection_buttons},
    },
};
use glam::Vec2;
use std::{cell::RefCell, rc::Rc};

pub fn get(data: Rc<RefCell<Data>>) -> ComputedWidgetNode {
    let board = component::board::make_board(
        FSize::from(Vec2::new(32.0 * 16.0, 32.0 * 16.0)),
        data.clone(),
    )
    .finalize();

    let spacer1 = WidgetBuilder::default()
        .size(FSize::new(0.0, 32.0))
        .finalize();

    let promotion_selection_buttons =
        make_promotion_selection_buttons(FSize::new(32.0, 32.0), data.clone()).finalize();

    let spacer2 = WidgetBuilder::default()
        .size(FSize::new(0.0, 32.0))
        .finalize();

    let debug_toggle_button =
        component::debug::make_toggle_button(FSize::new(100.0, 100.0)).finalize();

    let root = WidgetBuilder::new_col(true)
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
