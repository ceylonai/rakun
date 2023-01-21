use std::fmt;
use pyo3::{Py, PyAny};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub data: Py<PyAny>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message: {} {}", self.id, self.data)
    }
}