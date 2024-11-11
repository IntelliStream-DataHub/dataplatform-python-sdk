use std::collections::HashMap;
use pyo3::prelude::*;
use crate::credentials::CredentialProvider;

#[pyclass]
#[derive(Clone)]
pub struct ClientConfig {

    #[pyo3(get, set)]
    client_name: String,

    #[pyo3(get, set)]
    project: String,

    #[pyo3(get, set)]
    base_url: Option<String>,

    #[pyo3(get, set)]
    headers: Option<HashMap<String, String>>,

    #[pyo3(get, set)]
    timeout: Option<u64>,

    #[pyo3(get, set)]
    file_transfer_timeout: Option<u64>,

    #[pyo3(get, set)]
    credentials: CredentialProvider,
}

#[pymethods]
impl ClientConfig {
    #[new]
    pub fn new(
        client_name: String,
        project: String,
        credentials: CredentialProvider,
        base_url: Option<String>,
        headers: Option<HashMap<String, String>>,
        timeout: Option<u64>,
        file_transfer_timeout: Option<u64>,
    ) -> Self {
        ClientConfig { client_name, project, base_url, headers, timeout, file_transfer_timeout, credentials }
    }

}

pub fn init_config_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Assuming Bound provides a method to get the Python interpreter
    let py = parent_module.py();

    // Create the 'config' submodule

    let config_module = PyModule::new_bound(py, "config")?;

    // Add the 'ClientConfig' class to the 'config' submodule
    config_module.add_class::<ClientConfig>()?;

    // Add the 'config' submodule to the parent module
    parent_module.add_submodule(&config_module)?;

    Ok(())
}