#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
    pub _private: (),
}

impl Coordinate {
    pub const fn new(row: i32, col: i32) -> Option<Self> {
        if row < 0 || row > 15 || col < 0 || col > 15 {
            return None;
        }

        Some(Self {
            row: row as usize,
            col: col as usize,
            _private: (),
        })
    }

    pub const fn new_unchecked(row: i32, col: i32) -> Self {
        assert!(!(row < 0 || row > 15 || col < 0 || col > 15));
        Self {
            row: row as usize,
            col: col as usize,
            _private: (),
        }
    }

    pub const fn zero() -> Self {
        Self {
            row: 0,
            col: 0,
            _private: (),
        }
    }

    pub const fn offset(&self, direction: Direction) -> Option<Self> {
        Self::new(
            self.row as i32 + direction.row,
            self.col as i32 + direction.col,
        )
    }
}

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

impl std::ops::Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.row, -self.col)
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
