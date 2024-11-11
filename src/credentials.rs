use pyo3::{pyclass, pymethods, Bound, PyResult};
use pyo3::prelude::{PyModule, PyModuleMethods};
use crate::clients::DataPlatformClient;

#[pyclass]
#[derive(Clone)]
pub struct CredentialProvider {

}

#[pymethods]
impl CredentialProvider {

}

pub fn init_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent_module.py();
    let new_module = PyModule::new_bound(py, "credentials")?;
    new_module.add_class::<CredentialProvider>()?;
    parent_module.add_submodule(&new_module)?;
    Ok(())
}
