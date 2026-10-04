use crate::definition::engine::{Area, PieceRich, TileRich, tile};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoardView {
    pub rows: Vec<BoardRow>,
    pub tiles: Vec<TileRich>,
    pub pieces: Vec<PieceRich>,
    pub special_tile_pairs: Vec<tile::SpecialPair>,
    pub promotion_area: Area,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoardRow(pub Vec<TileRich>);
