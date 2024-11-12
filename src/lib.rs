use pyo3::prelude::*;

mod clients;
mod config;
mod credentials;

mod dataplatform_api;

#[pymodule]
fn dataplatform(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // Initialize and add the 'config' submodule
    config::init_module(m)?;

    // Initialize and add the 'clients' submodule
    clients::init_module(m)?;

    // Initialize and add the 'credentials' submodule
    credentials::init_module(m)?;

    Ok(())
}
