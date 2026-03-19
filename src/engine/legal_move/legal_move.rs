use crate::engine::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NormalMove {
    pub origin: Coordinate,
    pub destination: Coordinate,
}
