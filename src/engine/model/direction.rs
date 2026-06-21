#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Direction {
    pub row: i32,
    pub col: i32,
}

impl Direction {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

// BE CAREFUL ABOUT THE ORDER
const DIRECTION_STORE: [Direction; 16] = [
    // Rook
    Direction::new(1, 0),
    Direction::new(0, 1),
    Direction::new(0, -1),
    Direction::new(-1, 0),
    // Bishop
    Direction::new(1, 1),
    Direction::new(1, -1),
    Direction::new(-1, 1),
    Direction::new(-1, -1),
    // Knight
    Direction::new(2, 1),
    Direction::new(2, -1),
    Direction::new(1, 2),
    Direction::new(1, -2),
    Direction::new(-1, 2),
    Direction::new(-1, -2),
    Direction::new(-2, 1),
    Direction::new(-2, -1),
];

pub fn queen() -> &'static [Direction] {
    &DIRECTION_STORE[0..8]
}

pub fn rook() -> &'static [Direction] {
    &DIRECTION_STORE[0..4]
}

pub fn bishop() -> &'static [Direction] {
    &DIRECTION_STORE[4..8]
}

pub fn knight() -> &'static [Direction] {
    &DIRECTION_STORE[8..16]
}

////////////////////
// std::ops impls //
////////////////////

impl std::ops::Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.row, -self.col)
    }
}

impl std::ops::Sub for Direction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl std::ops::SubAssign for Direction {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

impl std::ops::Mul<i32> for Direction {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.row * rhs, self.col * rhs)
    }
}

impl std::ops::MulAssign<i32> for Direction {
    fn mul_assign(&mut self, rhs: i32) {
        self.row *= rhs;
        self.col *= rhs;
    }
}
