use std::sync::Arc;

use pyo3::prelude::*;

use crate::handlers::events::{EventHandler};

#[pyclass]
pub struct Agent {
    pub event_handler: Arc<EventHandler>,
}

#[pymethods]
impl Agent {
    #[new]
    fn __new__() -> Self {
        Agent {
            event_handler: Arc::new(EventHandler::new()),
        }
    }
}


