#![allow(unused_imports)]
pub use crate::engine::gamestate::Gamestate;
pub use crate::engine::model::chess_move::Move;
pub use crate::engine::model::coordinate::Coordinate;

pub mod legal_move {
    pub use crate::engine::model::chess_move::Castle;
    pub use crate::engine::model::chess_move::Defection;
    pub use crate::engine::model::chess_move::NormalMove;
    pub use crate::engine::model::chess_move::Promotion;
    pub use crate::engine::model::chess_move::RegimeChangePromotion;
}

pub mod faction {
    pub use crate::engine::model::faction::Color;
    pub use crate::engine::model::faction::Color::*;
}

pub mod piece {
    pub use crate::engine::model::piece::Type;
    pub use crate::engine::model::piece::Type::*;
}

pub mod tile {
    pub use crate::engine::model::tile::Special;
}
