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