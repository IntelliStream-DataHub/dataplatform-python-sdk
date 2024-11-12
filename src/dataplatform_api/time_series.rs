use pyo3::{pyclass, pymethods, Bound, PyResult};
use pyo3::types::PyType;

#[pyclass]
#[derive(Clone)]
pub struct DatapointsAPI{
}

#[pymethods]
impl DatapointsAPI{

    #[new]
    pub fn new() -> Self {
        DatapointsAPI{}
    }
}

#[pyclass]
#[derive(Clone)]
pub struct TimeSeriesAPI{
    resource_path: String,
    #[pyo3(get, set)]
    data: DatapointsAPI
}

#[pymethods]
impl TimeSeriesAPI{

    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> Self {
        let resource_path = String::from("/timeseries");
        TimeSeriesAPI{ resource_path, data: DatapointsAPI::new() }
    }

    #[pyo3(signature = (
        id=None,
        external_id=None
    ))]
    #[classmethod]
    pub fn retrieve(_cls: &Bound<PyType>, id: Option<u64>, external_id: Option<String>) -> PyResult<String> {
        Ok("{}".to_string())
    }
}