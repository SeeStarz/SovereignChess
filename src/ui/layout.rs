use crate::geometry::{FPosition, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Positioning {
    Fixed(FPosition, CardinalAnchor),    // Fixed to global coordinates
    Relative(FPosition, CardinalAnchor), // Offset relative to parent's coordinates
    Offset(FPosition, FlexAnchor),       // Offset relative to natural positioning as flex children
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardinalAnchor {
    TopLeft,    Top,    TopRight,
    Left,       Center, Right,
    BottomLeft, Bottom, BottomRight,
}

impl CardinalAnchor {
    pub fn fraction_offset_from_topleft(&self) -> FPosition {
        use CardinalAnchor::*;

        let left_offset = match self {
            TopLeft | Left | BottomLeft => 0.0,
            Top | Center | Bottom => 0.5,
            TopRight | Right | BottomRight => 1.0,
        };

        let top_offset = match self {
            TopLeft | Top | TopRight => 0.0,
            Left | Center | Right => 0.5,
            BottomLeft | Bottom | BottomRight => 1.0,
        };

        FPosition::new(left_offset, top_offset)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexAnchor {
    LeftOrTop,
    Center,
    RightOrBottom,
}

impl FlexAnchor {
    pub fn fraction_padding_from_topleft(&self) -> f32 {
        use FlexAnchor::*;

        match self {
            LeftOrTop => 0.0,
            Center => 0.5,
            RightOrBottom => 1.0,
        }
    }
}

type GrowFactor = u32;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlexDirection {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl FlexDirection {
    pub fn major_axis(self) -> Axis {
        use FlexDirection::*;
        match self {
            Right | Left => Axis::Horizontal,
            Up | Down => Axis::Vertical,
        }
    }

    pub fn minor_axis(self) -> Axis {
        use FlexDirection::*;
        match self {
            Right | Left => Axis::Vertical,
            Up | Down => Axis::Horizontal,
        }
    }
}

pub trait AxisIndexable<T> {
    fn get_on_axis(&self, axis: Axis) -> T;
    fn set_on_axis(&mut self, value: T, axis: Axis);
}

impl<T: Copy> AxisIndexable<T> for Size<T> {
    fn get_on_axis(&self, axis: Axis) -> T {
        use Axis::*;
        match axis {
            Horizontal => self.width,
            Vertical => self.height,
        }
    }

    fn set_on_axis(&mut self, value: T, axis: Axis) {
        use Axis::*;
        match axis {
            Horizontal => self.width = value,
            Vertical => self.height = value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisSizingRequest {
    Shrink,             // Child must eventually have a computable size
    Expand(GrowFactor), // Parent must eventually have a computable size
    Fixed(f32),
}

pub fn major_minor_to_size<T>(major: T, minor: T, major_axis: Axis) -> Size<T> {
    if major_axis == Axis::Horizontal {
        Size {
            width: major,
            height: minor,
        }
    } else {
        Size {
            width: minor,
            height: major,
        }
    }
}

pub fn size_to_major_minor<T>(size: Size<T>, major_axis: Axis) -> (T, T) {
    if major_axis == Axis::Horizontal {
        (size.width, size.height)
    } else {
        (size.height, size.width)
    }
}
