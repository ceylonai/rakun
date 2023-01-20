use log::{debug};
use pyo3::{prelude::*};

#[derive(Debug)]
#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    pub domain: String,
}

#[pymethods]
impl Agent {
    #[new]
    fn new(_py: Python<'_>, domain: String, id: Option<String>) -> Self {
        debug!("Initializing Agent");

        // Create domain with Id fo there is one
        let domain = match id {
            Some(id) => format!("{}:{}", domain, id),
            None => domain,
        };

        Agent {
            domain,
        }
    }

    fn register_event_handler(&self, _py: Python<'_>, name: String, handler: Py<PyAny>) -> PyResult<()> {
        debug!("Registering event {:?}, {:?}",name, handler);
        Ok(())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Agent: {}", self.domain))
    }
}

