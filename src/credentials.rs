use pyo3::{pyclass, pymethods, Bound, PyResult};
use pyo3::prelude::{PyModule, PyModuleMethods};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct CredentialProvider {

}

#[pymethods]
impl CredentialProvider {

    #[new]
    fn new() -> Self {
        CredentialProvider {  }
    }

}

#[pyclass(extends=CredentialProvider, subclass)]
pub struct TokenProvider {
    #[pyo3(get, set)]
    token: String
}

#[pymethods]
impl TokenProvider {

    #[new]
    fn new(token: String) -> (Self, CredentialProvider) {
        (TokenProvider { token }, CredentialProvider::new())
    }

}

#[pyclass(extends=CredentialProvider, subclass)]
pub struct OAuthClientCredentials {
    #[pyo3(get, set)]
    token_url: String,
    #[pyo3(get, set)]
    client_id: String,
    #[pyo3(get, set)]
    client_secret: String,
    #[pyo3(get, set)]
    scopes: Vec<String>,
    #[pyo3(get, set)]
    token_expiry_leeway_seconds: u64
}

#[pymethods]
impl OAuthClientCredentials {

    #[new]
    #[pyo3(signature = (
        token_url,
        client_id,
        client_secret,
        scopes=None,
    ))]
    fn new(
        token_url: String,
        client_id: String,
        client_secret: String,
        scopes: Option<Vec<String>>
    ) -> (Self, CredentialProvider) {

        (OAuthClientCredentials {
            token_url,
            client_id,
            client_secret,
            scopes: scopes.or_else(||
                Some(vec![
                    "openid".to_string(),
                    "profile".to_string(),
                    "email".to_string()]
                )).unwrap(),
            token_expiry_leeway_seconds: 30
        }, CredentialProvider::new())
    }

}

pub fn init_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent_module.py();
    let new_module = PyModule::new_bound(py, "credentials")?;
    new_module.add_class::<CredentialProvider>()?;
    new_module.add_class::<TokenProvider>()?;
    new_module.add_class::<OAuthClientCredentials>()?;
    parent_module.add_submodule(&new_module)?;
    Ok(())
}
