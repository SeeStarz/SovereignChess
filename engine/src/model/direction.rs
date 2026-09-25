use crate::Vec2;

// BE CAREFUL ABOUT THE ORDER
const DIRECTION_STORE: [Vec2; 16] = [
    // Rook
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(0, -1),
    Vec2::new(-1, 0),
    // Bishop
    Vec2::new(1, 1),
    Vec2::new(1, -1),
    Vec2::new(-1, 1),
    Vec2::new(-1, -1),
    // Knight
    Vec2::new(2, 1),
    Vec2::new(2, -1),
    Vec2::new(1, 2),
    Vec2::new(1, -2),
    Vec2::new(-1, 2),
    Vec2::new(-1, -2),
    Vec2::new(-2, 1),
    Vec2::new(-2, -1),
];

pub fn queen() -> &'static [Vec2] {
    &DIRECTION_STORE[0..8]
}

pub fn rook() -> &'static [Vec2] {
    &DIRECTION_STORE[0..4]
}

pub fn bishop() -> &'static [Vec2] {
    &DIRECTION_STORE[4..8]
}

pub fn knight() -> &'static [Vec2] {
    &DIRECTION_STORE[8..16]
}
