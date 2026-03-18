use crate::geometry::IRect;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Layout {
    Fixed(IRect),
    Relative(IRect),
}
