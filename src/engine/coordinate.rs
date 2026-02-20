#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl Coordinate {
    pub const fn new(row: i32, col: i32) -> Self {
        Coordinate { row, col }
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl std::ops::Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Coordinate::new(self.row, -self.col)
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Coordinate) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Coordinate) {
        *self = *self - rhs;
    }
}
