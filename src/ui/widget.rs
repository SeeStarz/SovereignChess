use crate::{IRect, ui::Layout};
use glam::IVec2;
use std::rc::Rc;

type Input = ();
type InputHandler = Rc<dyn Fn(&Input) -> bool>;

#[derive(Clone)]
pub struct WidgetIntent {
    pub children: Vec<WidgetIntent>,
    pub layout: Layout,
    pub input_handler: Option<InputHandler>,
}

impl WidgetIntent {
    pub fn compute(&self, parent: IRect) -> ComputedWidget {
        use Layout::*;
        let rect = match self.layout {
            Fixed(rect) => rect,
            Relative(rect) => IRect {
                position: (IVec2::from(rect.position) + IVec2::from(parent.position)).into(),
                size: rect.size,
            },
        };

        let children = self.children.iter().map(|c| c.compute(rect)).collect();
        ComputedWidget {
            children,
            rect,
            input_handler: self.input_handler.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ComputedWidget {
    pub children: Vec<ComputedWidget>,
    pub rect: IRect,
    pub input_handler: Option<InputHandler>,
}

impl ComputedWidget {
    /// Recursively calls handle_input on existing input_handler until a widget consumes it
    pub fn handle_input(&self, input: &Input) -> bool {
        self.children.iter().rev().any(|c| c.handle_input(input))
            || self.input_handler.as_ref().map_or(false, |f| f(input))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Self> {
        std::iter::once(self).chain(self.children.iter())
    }
}
