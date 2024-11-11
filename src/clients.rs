use pyo3::prelude::*;
use crate::config::ClientConfig;

#[pyclass]
pub struct DataPlatformClient {
    #[pyo3(get, set)]
    config: ClientConfig,
}

#[pymethods]
impl DataPlatformClient {
    #[new]
    pub fn new(config: ClientConfig) -> Self {
        DataPlatformClient { config }
    }
}


pub fn init_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent_module.py();
    let new_module = PyModule::new_bound(py, "clients")?;
    new_module.add_class::<DataPlatformClient>()?;
    parent_module.add_submodule(&new_module)?;
    Ok(())
}