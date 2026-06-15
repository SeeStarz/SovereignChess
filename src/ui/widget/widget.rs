use crate::{
    geometry::{FPosition, FRect, FSize, Size},
    ui::{
        input::Event,
        layout::{
            AxisIndexable, AxisSizingRequest, CardinalAnchor, FlexDirection, Positioning,
            major_minor_to_size, size_to_major_minor,
        },
    },
};
use glam::Vec2;
use raylib::{RaylibThread, core::drawing::RaylibDrawHandle};

pub type InputHandler = Box<dyn FnMut(Event, FRect) -> bool + 'static>;
pub type RenderFunction = Box<dyn Fn(&mut RaylibDrawHandle, &RaylibThread, FRect) + 'static>;

pub fn ignore_input(_input: Event, _rect: FRect) -> bool {
    false
}

pub fn no_render(_handle: &mut RaylibDrawHandle, _thread: &RaylibThread, _rect: FRect) {}

pub type WidgetSizeRequest = Size<AxisSizingRequest>;

pub struct WidgetSpecNode {
    pub children: Vec<WidgetSpecNode>,
    pub core: WidgetSpec,
}

pub struct WidgetSpec {
    pub size_request: WidgetSizeRequest,
    pub flex_direction: FlexDirection,
    pub positioning: Positioning,
    pub child_origin: CardinalAnchor,
    pub input_handler: InputHandler,
    pub render_function: RenderFunction,
}

pub struct ComputedWidgetNode {
    pub children: Vec<ComputedWidgetNode>,
    pub rect: FRect,
    pub spec: WidgetSpec,
}

impl WidgetSpecNode {
    pub fn compute(self) -> ComputedWidgetNode {
        let mut work = self.measure_intrinsic_size();

        let size = work.cache.size.unwrap_or_else(|_| panic!("Expand as root"));
        work.allocate_remaining_size(size);

        work.position_widgets(FPosition::default())
    }
}

impl ComputedWidgetNode {
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

///////////////////////////
// IMPLEMENTATION DETAIL //
///////////////////////////

type OptFSize = Size<Option<f32>>;

impl OptFSize {
    fn unwrap_or_else<F>(self, closure: F) -> FSize
    where
        F: FnOnce(OptFSize) -> FSize,
    {
        let Some(width) = self.width else {
            return closure(self);
        };
        let Some(height) = self.height else {
            return closure(self);
        };
        FSize::new(width, height)
    }

    fn unwrap(self) -> FSize {
        self.unwrap_or_else(|_opt_fsize| panic!())
    }
}

struct WidgetWorkCache {
    size: OptFSize,
    available_size: OptFSize,
}

struct WidgetIntermediaryNode {
    spec: WidgetSpec,
    children: Vec<WidgetIntermediaryNode>,
    cache: WidgetWorkCache,
}

impl WidgetSpecNode {
    fn measure_intrinsic_size(self) -> WidgetIntermediaryNode {
        let children: Vec<WidgetIntermediaryNode> = self
            .children
            .into_iter()
            .map(|c| c.measure_intrinsic_size())
            .collect();
        let cache = {
            let major_axis = self.core.flex_direction.major_axis();
            let minor_axis = self.core.flex_direction.minor_axis();

            use AxisSizingRequest::*;

            let major_length = match self.core.size_request.get_on_axis(major_axis) {
                Fixed(length) => Some(length),
                Expand(_) => None,
                Shrink => Some(children.iter().fold(0.0, |acc, c| {
                    acc + c
                        .cache
                        .size
                        .get_on_axis(major_axis)
                        .expect("Unresolved size for shrink children")
                })),
            };

            assert!(
                major_length != Some(0.0)
                    || self.core.size_request.get_on_axis(major_axis) != Shrink
            );

            let minor_length = match self.core.size_request.get_on_axis(minor_axis) {
                Fixed(length) => Some(length),
                Expand(_) => None,
                Shrink => Some(children.iter().fold(0.0, |acc, c| {
                    (acc as f32).max(c.cache.size.get_on_axis(minor_axis).unwrap_or(0.0))
                })),
            };

            assert!(
                minor_length != Some(0.0)
                    || self.core.size_request.get_on_axis(minor_axis) != Shrink
            );

            WidgetWorkCache {
                size: major_minor_to_size(major_length, minor_length, major_axis),
                available_size: Size::new(None, None),
            }
        };

        WidgetIntermediaryNode {
            spec: self.core,
            children,
            cache,
        }
    }
}

impl WidgetIntermediaryNode {
    fn store_given_size(&mut self, given_size: FSize) {
        use AxisSizingRequest::*;

        self.cache.available_size.width = Some(given_size.width);
        self.cache.available_size.height = Some(given_size.height);

        match self.spec.size_request.width {
            Fixed(_) | Shrink => {}
            Expand(_grow_factor) => self.cache.size.width = Some(given_size.width),
        }

        match self.spec.size_request.height {
            Fixed(_) | Shrink => {}
            Expand(_grow_factor) => {
                self.cache.size.height = Some(given_size.height);
            }
        }
    }

    fn allocate_remaining_size(&mut self, given_size: FSize) {
        self.store_given_size(given_size);

        let major_axis = self.spec.flex_direction.major_axis();

        let (major_length, minor_length) =
            size_to_major_minor(self.cache.size.unwrap(), major_axis);

        // None variant should mean the child is of type expand
        let used_major_length = self.children.iter().fold(0.0, |acc, c| {
            acc + c.cache.size.get_on_axis(major_axis).unwrap_or(0.0)
        });

        let usable_major_length = major_length - used_major_length;

        use AxisSizingRequest::*;

        let total_growth_factor = self.children.iter().fold(0, |acc, c| {
            acc + match c.spec.size_request.get_on_axis(major_axis) {
                Expand(grow_factor) => grow_factor,
                _ => 0,
            }
        });

        // Either the widget should have not overflown
        // or it was probably not intended to care about sizing in the first place
        assert!(usable_major_length >= 0.0 || total_growth_factor == 0);

        self.children.iter_mut().for_each(|c| {
            let child_major_length = match c.spec.size_request.get_on_axis(major_axis) {
                Fixed(_width) => c.cache.size.get_on_axis(major_axis).unwrap(),
                Shrink => c.cache.size.get_on_axis(major_axis).unwrap(),
                Expand(grow_factor) => {
                    usable_major_length / total_growth_factor as f32 * grow_factor as f32
                }
            };

            c.allocate_remaining_size(major_minor_to_size(
                child_major_length,
                minor_length,
                major_axis,
            ));
        });
    }

    fn position_widgets(self, origin: FPosition) -> ComputedWidgetNode {
        use Positioning::*;
        let rect = {
            let size = self.cache.size.unwrap();

            let position = FPosition::from({
                match self.spec.positioning {
                    Absolute(position, anchor) => {
                        Vec2::from(position)
                            - Vec2::from(size) * Vec2::from(anchor.fraction_offset_from_topleft())
                    }
                    Relative(offset, anchor) => {
                        Vec2::from(origin) + Vec2::from(offset)
                            - Vec2::from(size) * Vec2::from(anchor.fraction_offset_from_topleft())
                    }
                    Offset(offset, anchor) => {
                        let padding = Vec2::from(self.cache.available_size.unwrap())
                            - Vec2::from(self.cache.size.unwrap());
                        Vec2::from(origin)
                            + Vec2::from(offset)
                            + padding * anchor.fraction_padding_from_topleft()
                    }
                }
            });

            FRect { size, position }
        };

        let major_axis = self.spec.flex_direction.major_axis();
        let mut accumulated_major_length = 0.0;

        let children: Vec<ComputedWidgetNode> = {
            self.children
                .into_iter()
                .map(|c| {
                    let origin = match c.spec.positioning {
                        Absolute(_, _) | Relative(_, _) => {
                            Vec2::from(rect.position)
                                + Vec2::from(rect.size)
                                    * Vec2::from(
                                        self.spec.child_origin.fraction_offset_from_topleft(),
                                    )
                        }

                        Offset(_, _) => {
                            Vec2::from(rect.position)
                                + Vec2::from(major_minor_to_size(
                                    accumulated_major_length,
                                    0.0,
                                    major_axis,
                                ))
                        }
                    };
                    accumulated_major_length += c.cache.size.get_on_axis(major_axis).unwrap();
                    c.position_widgets(FPosition::from(origin))
                })
                .collect()
        };

        ComputedWidgetNode {
            children,
            rect,
            spec: self.spec,
        }
    }
}
