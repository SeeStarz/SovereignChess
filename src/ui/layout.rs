use crate::geometry::FRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    Fixed(FRect),
    Relative(FRect),
}
