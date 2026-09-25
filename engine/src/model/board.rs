use std::collections::HashMap;

use crate::{
    Coordinate, PieceSimple, PieceWithCoordinate, TileSimple, model::geometry::Area,
    tile::TileWithCoordinate,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    tiles: Vec<TileSimple>,
    width: u32,
    height: u32,
    promotion_area: Area,
}

impl Board {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn promotion_area(&self) -> Area {
        self.promotion_area
    }

    pub fn tiles(&self) -> impl Iterator<Item = TileWithCoordinate> {
        (0..self.height).flat_map(move |r| {
            (0..self.width).map(move |c| {
                let coordinate = Coordinate::new(r as i32, c as i32);
                let tile = self.at(coordinate).unwrap();
                let piece = tile
                    .0
                    .map(|p| PieceWithCoordinate::from_simple(p, coordinate));
                TileWithCoordinate { coordinate, piece }
            })
        })
    }

    pub fn pieces(&self) -> impl Iterator<Item = PieceWithCoordinate> {
        self.tiles().filter_map(|t| t.piece)
    }

    pub fn empty(width: u32, height: u32, promotion_area: Area) -> Option<Board> {
        if promotion_area.left < 0 || promotion_area.top < 0 {
            return None;
        }
        if promotion_area.right as u32 >= width || promotion_area.bottom as u32 >= height {
            return None;
        }

        Some(Board {
            tiles: (0..(width * height)).map(|_| TileSimple(None)).collect(),
            width,
            height,
            promotion_area,
        })
    }

    pub fn from_piece_hashmap(
        map: &HashMap<Coordinate, PieceSimple>,
        width: u32,
        height: u32,
        promotion_area: Area,
    ) -> Option<Board> {
        let mut board = Self::empty(width, height, promotion_area)?;
        if !map
            .into_iter()
            .all(|(&coordinate, &piece)| board.set_at(coordinate, Some(piece)))
        {
            return None;
        }
        Some(board)
    }

    pub fn is_coordinate_valid(&self, coordinate: Coordinate) -> bool {
        coordinate.row >= 0
            && coordinate.row < self.height as i32
            && coordinate.col >= 0
            && coordinate.col < self.width as i32
    }

    pub fn at(&self, coordinate: Coordinate) -> Option<TileSimple> {
        if !self.is_coordinate_valid(coordinate) {
            return None;
        }

        Some(self.tiles[(self.height as i32 * coordinate.row + coordinate.col) as usize])
    }

    pub fn set_at(&mut self, coordinate: Coordinate, piece: Option<PieceSimple>) -> bool {
        if !self.is_coordinate_valid(coordinate) {
            return false;
        }

        self.tiles[(self.height as i32 * coordinate.row + coordinate.col) as usize] =
            TileSimple(piece);
        true
    }
}
