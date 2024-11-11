use pyo3::prelude::*;

mod clients;
mod config;
mod credentials;

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok("Hello from Rust!".to_string())
}

#[pymodule]
fn dataplatform_sdk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;

    // Initialize and add the 'config' submodule
    config::init_config_module(m)?;

    Ok(())
}

