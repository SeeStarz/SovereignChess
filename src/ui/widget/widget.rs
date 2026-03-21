use crate::{
    geometry::FRect,
    ui::{Layout, input::Event},
};
use glam::Vec2;
use raylib::{RaylibThread, core::drawing::RaylibDrawHandle};

pub type InputHandler = Box<dyn FnMut(Event, FRect) -> bool + 'static>;
pub type RenderFunction = Box<dyn Fn(&mut RaylibDrawHandle, &RaylibThread, FRect) + 'static>;

pub fn ignore_input(_input: Event, _rect: FRect) -> bool {
    false
}

pub fn no_render(_handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _rect: FRect) {}

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
            layout: self.layout,
            rect,
            input_handler: self.input_handler,
            render_function: self.render_function,
        }
    }
}

pub struct ComputedWidget {
    pub children: Vec<ComputedWidget>,
    pub layout: Layout,
    pub rect: FRect,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

impl ComputedWidget {
    /// Recursively calls handle_input on existing input_handler until a widget consumes it
    pub fn handle_input(&mut self, event: Event) -> bool {
        self.children
            .iter_mut()
            .rev()
            .any(|c| c.handle_input(event))
            || (self.input_handler)(event, self.rect)
    }

    pub fn render(&self, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {
        (self.render_function)(handle, thread, self.rect);
        self.children.iter().for_each(|c| c.render(handle, thread))
    }

    pub fn recompute(self, parent: FRect) -> ComputedWidget {
        use Layout::*;
        let rect = match self.layout {
            Fixed(rect) => rect,
            Relative(rect) => FRect {
                position: (Vec2::from(rect.position) + Vec2::from(parent.position)).into(),
                size: rect.size,
            },
        };

        let children = self
            .children
            .into_iter()
            .map(|c| c.recompute(rect))
            .collect();
        ComputedWidget {
            children,
            layout: self.layout,
            rect,
            input_handler: self.input_handler,
            render_function: self.render_function,
        }
    }
}
