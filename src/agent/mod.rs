use log::{debug, info};
use pyo3::{prelude::*};

#[pyclass(subclass)]
pub struct Agent {}

#[pymethods]
impl Agent {
    #[new]
    fn new() -> Self {
        debug!("Initializing Agent");
        Agent {}
    }
}