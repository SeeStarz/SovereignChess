use crate::geometry::FPosition;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    MousePressed(FPosition),
}
