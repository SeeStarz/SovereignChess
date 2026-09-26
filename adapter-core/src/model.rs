#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub struct GameState {
    pub board: Vec<Tile>,
    pub variant_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub struct Tile {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub special_faction: Option<FactionId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub struct Piece {
    pub faction: FactionId,
    pub owner: Option<FactionId>,
    pub piece_type: PieceType,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub struct FactionId(pub u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "pyo3", pyclass(from_py_object))]
pub struct Coordinate(pub u32, pub u32);

///////////////////
// pyo3 bindings //
///////////////////
#[cfg(feature = "pyo3")]
mod pyo3_bindings {
    use super::*;
    use pyo3::prelude::*;
    #[pymethods]
    impl Coordinate {
        #[new]
        fn __new__(row: u32, col: u32) -> Self {
            Self(row, col)
        }

        fn __repr__(&self) -> String {
            format!("{:?}", self)
        }
    }

    #[pymethods]
    impl FactionId {
        #[new]
        fn __new__(id: u32) -> Self {
            Self(id)
        }

        fn __repr__(&self) -> String {
            format!("{:?}", self)
        }
    }
}

//////////////////////
// impl From engine //
//////////////////////
impl From<engine::Coordinate> for Coordinate {
    fn from(value: engine::Coordinate) -> Self {
        Self(value.row as u32, value.col as u32)
    }
}

impl From<engine::FactionId> for FactionId {
    fn from(value: engine::FactionId) -> Self {
        Self(value.0)
    }
}

impl From<engine::piece::Type> for PieceType {
    fn from(value: engine::piece::Type) -> Self {
        use engine::piece::Type as E;
        match value {
            E::King => Self::King,
            E::Queen => Self::Queen,
            E::Rook => Self::Rook,
            E::Bishop => Self::Bishop,
            E::Knight => Self::Knight,
            E::Pawn => Self::Pawn,
        }
    }
}

impl From<engine::PieceRich> for Piece {
    fn from(value: engine::PieceRich) -> Self {
        Self {
            faction: FactionId::from(value.faction),
            owner: value.owner.map(|f| FactionId::from(f)),
            piece_type: PieceType::from(value.piece_type),
            coordinate: Coordinate::from(value.coordinate),
        }
    }
}

impl From<engine::TileRich> for Tile {
    fn from(value: engine::TileRich) -> Self {
        Self {
            coordinate: Coordinate::from(value.coordinate),
            piece: value.piece.map(|p| Piece::from(p)),
            special_faction: value.special.map(|s| FactionId::from(s.faction())),
        }
    }
}

impl From<engine::GameState> for GameState {
    fn from(value: engine::GameState) -> Self {
        Self {
            board: engine::logic::board_tiles_rich(&value)
                .map(|t| Tile::from(t))
                .collect(),
            variant_name: value.variant_data.variant_name,
        }
    }
}
