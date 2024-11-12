use pyo3::prelude::*;
use crate::config::ClientConfig;
use crate::dataplatform_api::time_series::TimeSeriesAPI;

#[pyclass]
pub struct DataPlatformClient {
    #[pyo3(get, set)]
    config: ClientConfig,
    #[pyo3(get, set)]
    time_series: TimeSeriesAPI
}

#[pymethods]
impl DataPlatformClient {
    #[new]
    pub fn new(config: ClientConfig) -> Self {
        let time_series = TimeSeriesAPI::new();
        DataPlatformClient { config, time_series }
    }
}


pub fn init_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent_module.py();
    let new_module = PyModule::new_bound(py, "clients")?;
    new_module.add_class::<DataPlatformClient>()?;
    parent_module.add_submodule(&new_module)?;
    Ok(())
}