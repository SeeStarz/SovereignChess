#![allow(unused_imports)]
pub use crate::engine::coordinate::Coordinate;
pub use crate::engine::gamestate::Gamestate;
pub use crate::engine::legal_move::LegalMove;

pub mod legal_move {
    pub use crate::engine::legal_move::Castle;
    pub use crate::engine::legal_move::Defection;
    pub use crate::engine::legal_move::NormalMove;
    pub use crate::engine::legal_move::Promotion;
    pub use crate::engine::legal_move::RegimeChangePromotion;
}

pub mod faction {
    pub use crate::engine::faction::Color;
    pub use crate::engine::faction::Color::*;
}

pub mod piece {
    pub use crate::engine::piece::Type;
    pub use crate::engine::piece::Type::*;
}

pub mod tile {
    pub use crate::engine::tile::Special;
}
