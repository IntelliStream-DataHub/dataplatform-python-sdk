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
    credentials: CredentialProvider,

    #[pyo3(get, set)]
    base_url: Option<String>,

    #[pyo3(get, set)]
    headers: Option<HashMap<String, String>>,

    #[pyo3(get, set)]
    timeout: Option<u64>,

    #[pyo3(get, set)]
    file_transfer_timeout: Option<u64>,

}

#[pymethods]
impl ClientConfig {

    #[new]
    #[pyo3(signature = (
        client_name,
        project,
        credentials,
        base_url=None,
        headers=None,
        timeout=None,
        file_transfer_timeout=None
    ))]
    pub fn new(
        client_name: String,
        project: String,
        credentials: CredentialProvider,
        base_url: Option<String>,
        headers: Option<HashMap<String, String>>,
        timeout: Option<u64>,
        file_transfer_timeout: Option<u64>,
    ) -> Self {
        ClientConfig {
            client_name, project, credentials,
            base_url: base_url.or(Some(String::from("intellistream.ai"))),
            headers: headers.or(Some(HashMap::new())),
            timeout: timeout.or(Some(30)),
            file_transfer_timeout: file_transfer_timeout.or(Some(30)),
        }
    }

}

pub fn init_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
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