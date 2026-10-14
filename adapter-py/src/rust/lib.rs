use pyo3::{exceptions::PyValueError, prelude::*};
use serde::de::DeserializeOwned;
use shared_model::definition::{
    adapter_core::SessionAction,
    engine::{VariantData, VariantPreset},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass(from_py_object)]
pub struct SessionAdapter {
    inner: adapter_core::SessionAdapter,
}

#[pymethods]
impl SessionAdapter {
    #[staticmethod]
    pub fn new(variant_data: &str) -> PyResult<Self> {
        let variant_data = deserialize_input::<VariantData>(variant_data)?;
        let inner = adapter_core::SessionAdapter::new(variant_data)
            .ok_or_else(|| PyValueError::new_err("Variant data invalid"))?;
        Ok(Self { inner })
    }

    #[staticmethod]
    pub fn new_from_preset(variant_preset: &str) -> PyResult<Self> {
        let variant_preset = deserialize_input::<VariantPreset>(variant_preset)?;
        let inner = adapter_core::SessionAdapter::new_from_preset(variant_preset);
        Ok(Self { inner })
    }

    pub fn do_action(&mut self, action: &str) -> PyResult<bool> {
        let action = deserialize_input::<SessionAction>(action)?;
        Ok(self.inner.do_action(action))
    }

    pub fn undo(&mut self) -> bool {
        self.inner.undo()
    }

    pub fn game_view(&self) -> String {
        serde_json::to_string(&self.inner.game_view()).expect("Failed to serialize game view")
    }
}

fn deserialize_input<T>(value: &str) -> PyResult<T>
where
    T: DeserializeOwned + ?Sized,
{
    serde_json::from_str::<T>(value).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
mod rust_bridge {
    #[pymodule_export]
    use super::SessionAdapter;
}
