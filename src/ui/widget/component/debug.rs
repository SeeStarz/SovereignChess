use crate::{
    geometry::FPosition,
    ui::{
        Layout, WidgetIntent,
        input::Event,
        widget::{InputHandler, RenderFunction},
    },
    util::Observer,
};
use raylib::{color::Color, math::Rectangle, prelude::RaylibDraw};
use std::{cell::RefCell, rc::Rc};

pub fn debug_rect(layout: Layout) -> WidgetIntent {
    let is_toggled = Rc::new(RefCell::new(false));
    let is_toggled_view = Observer::from(is_toggled.clone());

    let children = Vec::new();
    let input_handler: InputHandler = Box::new(move |event, rect| match event {
        Event::MousePressed(position)
            if Rectangle::from(rect).check_collision_point_rec(position) =>
        {
            let current = *is_toggled.borrow();
            *is_toggled.borrow_mut() = !current;
            true
        }
        _ => false,
    });
    let render_function: RenderFunction = Box::new(move |handle, _thread, rect| {
        handle.draw_rectangle_pro(
            rect,
            FPosition::default(),
            0.0,
            if *is_toggled_view.borrow() {
                Color::RED
            } else {
                Color::LIMEGREEN
            },
        );
    });

    WidgetIntent {
        children,
        layout,
        input_handler,
        render_function,
    }
}
