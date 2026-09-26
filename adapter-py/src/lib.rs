use pyo3::prelude::*;

#[pymodule]
mod adapter {
    #[pymodule_export]
    use adapter_core::model::{Coordinate, FactionId, GameState, Piece, PieceType, Tile};
}
