use crate::{
    geometry::{FPosition, FRect},
    ui::Layout,
};
use glam::Vec2;
use raylib::{RaylibThread, color::Color, core::drawing::RaylibDrawHandle, prelude::RaylibDraw};

type Input = (); // TODO
type InputHandler = Box<dyn FnMut(&Input) -> bool + 'static>;
type RenderFunction = Box<dyn Fn(&mut RaylibDrawHandle, &RaylibThread, FRect) + 'static>;

pub fn ignore_input(_input: &Input) -> bool {
    false
}

pub fn no_render(_handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _rect: FRect) {}

pub fn debug_rect(layout: Layout) -> WidgetIntent {
    let children = Vec::new();
    let input_handler = Box::new(ignore_input);
    let render_function: RenderFunction = Box::new(|handle, _thread, rect| {
        handle.draw_rectangle_pro(rect, FPosition::default(), 0.0, Color::RED);
    });

    WidgetIntent {
        children,
        layout,
        input_handler,
        render_function,
    }
}

pub struct WidgetIntent {
    pub children: Vec<WidgetIntent>,
    pub layout: Layout,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

impl WidgetIntent {
    pub fn compute(self, parent: FRect) -> ComputedWidget {
        use Layout::*;
        let rect = match self.layout {
            Fixed(rect) => rect,
            Relative(rect) => FRect {
                position: (Vec2::from(rect.position) + Vec2::from(parent.position)).into(),
                size: rect.size,
            },
        };

        let children = self.children.into_iter().map(|c| c.compute(rect)).collect();
        ComputedWidget {
            children,
            rect,
            input_handler: self.input_handler,
            render_function: self.render_function,
        }
    }
}

pub struct ComputedWidget {
    pub children: Vec<ComputedWidget>,
    pub rect: FRect,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

impl ComputedWidget {
    /// Recursively calls handle_input on existing input_handler until a widget consumes it
    pub fn handle_input(&mut self, input: &Input) -> bool {
        self.children
            .iter_mut()
            .rev()
            .any(|c| c.handle_input(input))
            || (self.input_handler)(input)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Self> {
        std::iter::once(self).chain(self.children.iter())
    }
}
