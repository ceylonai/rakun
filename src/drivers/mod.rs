use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(subclass)]
pub struct Driver {
    pub name: String,
}

#[pymethods]
impl Driver {
    #[new]
    fn __new__(name: String) -> Self {
        Driver {
            name,
        }
    }
}