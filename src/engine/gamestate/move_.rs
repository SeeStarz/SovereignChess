use crate::engine::export::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub origin: Coordinate,
    pub destination: Coordinate,
}
