use crate::engine::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleSource {
    pub king_coordinate: Coordinate,
    pub rook_coordinate: Coordinate,
}
