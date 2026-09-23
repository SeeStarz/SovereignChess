use glam::{IVec2, Vec2};
use raylib::{math::Rectangle, math::Vector2};

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
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<FPosition> for Vec2 {
    fn from(value: FPosition) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Vec2> for FPosition {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Vector2> for FPosition {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<FPosition> for raylib::ffi::Vector2 {
    fn from(value: FPosition) -> Self {
        Self {
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
        Self {
            x: value.width,
            y: value.height,
        }
    }
}
impl From<FSize> for Vec2 {
    fn from(value: FSize) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}
impl From<Vec2> for FSize {
    fn from(value: Vec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}
impl From<Vector2> for FSize {
    fn from(value: Vector2) -> Self {
        Self {
            width: value.x,
            height: value.y,
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
        Self {
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
impl From<FRect> for Rectangle {
    fn from(value: FRect) -> Self {
        Self {
            x: value.position.x,
            y: value.position.y,
            width: value.size.width,
            height: value.size.height,
        }
    }
}
