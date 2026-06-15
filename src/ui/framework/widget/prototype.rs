use crate::{
    geometry::{FPosition, FRect, Size},
    ui::framework::{
        input::Event,
        layout::{AxisSizingRequest, CardinalAnchor, FlexDirection, Positioning},
        widget,
    },
};
use raylib::{RaylibThread, core::drawing::RaylibDrawHandle};

pub type InputHandler = Box<dyn FnMut(Event, FRect) -> bool + 'static>;
pub type RenderFunction = Box<dyn Fn(&mut RaylibDrawHandle, &RaylibThread, FRect) + 'static>;

pub fn ignore_input(_input: Event, _rect: FRect) -> bool {
    false
}

pub fn no_render(_handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _rect: FRect) {}

pub type SizeRequest = Size<AxisSizingRequest>;

pub struct SpecNode {
    pub children: Vec<widget::SpecNode>,
    pub core: widget::Spec,
}

pub struct Spec {
    pub size_request: widget::SizeRequest,
    pub flex_direction: FlexDirection,
    pub positioning: Positioning,
    pub child_origin: CardinalAnchor,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

pub struct ComputedNode {
    pub children: Vec<widget::ComputedNode>,
    pub rect: FRect,
    pub spec: widget::Spec,
}

impl SpecNode {
    pub fn compute(self) -> widget::ComputedNode {
        let mut work = self.measure_intrinsic_size();
        work.allocate_remaining_size();
        work.position_widgets(FPosition::default())
    }
}

impl widget::ComputedNode {
    /// Recursively calls handle_input on existing input_handler until a widget consumes it
    pub fn handle_input(&mut self, event: Event) -> bool {
        self.children
            .iter_mut()
            .rev()
            .any(|c| c.handle_input(event))
            || (self.spec.input_handler)(event, self.rect)
    }

    pub fn render(&self, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {
        (self.spec.render_function)(handle, thread, self.rect);
        self.children.iter().for_each(|c| c.render(handle, thread))
    }
}
