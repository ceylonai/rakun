use log::{debug, info};
use pyo3::{prelude::*};

#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    domain: String,
}

#[pymethods]
impl Agent {
    #[new]
    fn new(py: Python<'_>, domain: String) -> Self {
        debug!("Initializing Agent");
        Agent {
            domain,
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Agent: {}", self.domain))
    }
}

