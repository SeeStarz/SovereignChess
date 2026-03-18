use glam::IVec2;
use raylib::ffi::Rectangle;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}
impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
pub type IPosition = Position<i32>;
pub type FPosition = Position<f32>;
impl From<IVec2> for IPosition {
    fn from(value: IVec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<IPosition> for IVec2 {
    fn from(value: IPosition) -> Self {
        IVec2 {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<FPosition> for raylib::ffi::Vector2 {
    fn from(value: FPosition) -> Self {
        raylib::ffi::Vector2 {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}
pub type ISize = Size<i32>;
pub type FSize = Size<f32>;

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}
impl From<IVec2> for ISize {
    fn from(value: IVec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}
impl From<ISize> for IVec2 {
    fn from(value: ISize) -> Self {
        IVec2 {
            x: value.width,
            y: value.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Rect<T> {
    pub position: Position<T>,
    pub size: Size<T>,
}
pub type IRect = Rect<i32>;
pub type FRect = Rect<f32>;
impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            position: Position { x, y },
            size: Size { width, height },
        }
    }
}
impl From<FRect> for raylib::ffi::Rectangle {
    fn from(value: FRect) -> Self {
        raylib::ffi::Rectangle {
            x: value.position.x,
            y: value.position.y,
            width: value.size.width,
            height: value.size.height,
        }
    }
}
impl From<Rectangle> for FRect {
    fn from(value: Rectangle) -> Self {
        Self {
            position: FPosition {
                x: value.x,
                y: value.y,
            },
            size: FSize {
                width: value.width,
                height: value.height,
            },
        }
    }
}
