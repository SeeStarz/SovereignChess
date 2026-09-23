use crate::geometry::FPosition;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    MousePressed(FPosition),
    _ExtensibilityMarker(private::Private),
}

mod private {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Private;
}
