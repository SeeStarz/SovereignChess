use crate::definition::engine::faction::{ColorStandard, FactionId};

impl From<ColorStandard> for FactionId {
    fn from(value: ColorStandard) -> Self {
        FactionId(value as u32)
    }
}
