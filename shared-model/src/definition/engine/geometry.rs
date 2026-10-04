#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub row: i32,
    pub col: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "python", pyclass(from_py_object))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Area {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
