use crate::engine::model::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub const fn row(&self) -> usize {
        self.row
    }

    pub const fn col(&self) -> usize {
        self.col
    }

    pub const fn new(row: i32, col: i32) -> Option<Self> {
        if row < 0 || row > 15 || col < 0 || col > 15 {
            return None;
        }

        Some(Self {
            row: row as usize,
            col: col as usize,
        })
    }

    pub const fn new_unchecked(row: i32, col: i32) -> Self {
        assert!(!(row < 0 || row > 15 || col < 0 || col > 15));
        Self {
            row: row as usize,
            col: col as usize,
        }
    }

    pub const fn offset(&self, direction: Direction) -> Option<Self> {
        Self::new(
            self.row as i32 + direction.row,
            self.col as i32 + direction.col,
        )
    }
}
