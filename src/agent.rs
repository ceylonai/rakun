use std::sync::Arc;

use pyo3::prelude::*;

use crate::handlers::events::{Event, EventHandler};

#[pyclass]
pub struct Agent {
    pub base_class: Arc<Py<PyAny>>,
    pub event_handler: Arc<EventHandler>,
}

#[pymethods]
impl Agent {
    fn new(base_class: Py<PyAny>) -> Self {
        Agent {
            base_class: Arc::new(base_class),
            event_handler: Arc::new(EventHandler::new()),
        }
    }

    fn add_event(&self, event_type: String, event: Event) {
        let mut event_handler = Arc::clone(&self.event_handler);
        event_handler.add_event(event_type, event);
    }
}


