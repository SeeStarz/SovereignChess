pub mod board;
mod castle_button;
pub mod debug;
mod defection_selection;
mod promotion_selection;

pub use board::build as board;
pub use castle_button::build as castle_button;
pub use defection_selection::build as defection_selection;
pub use promotion_selection::build as promotion_selection;
