#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl Coordinate {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub const fn offset(&self, direction: Vec2) -> Self {
        Self::new(self.row + direction.row, self.col + direction.col)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub row: i32,
    pub col: i32,
}

impl Vec2 {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn from_coordinate_pair(origin: Coordinate, destination: Coordinate) -> Self {
        Self {
            row: destination.row - origin.row,
            col: destination.col - origin.col,
        }
    }

    pub fn manhattan_distance(&self) -> u32 {
        self.row.abs() as u32 + self.col.abs() as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Area {
    pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn new_with_vec2(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self {
            left: top_left.col,
            right: bottom_right.col,
            top: top_left.row,
            bottom: bottom_right.row,
        }
    }
}

////////////////////
// std::ops impls //
////////////////////

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.row, -self.col)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.row * rhs, self.col * rhs)
    }
}

impl std::ops::MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.row *= rhs;
        self.col *= rhs;
    }
}
