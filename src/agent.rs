use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::handlers::events::{Event, EventHandler, EventType};

#[pyclass(subclass)]
pub struct Agent {
    pub base_class: Py<PyAny>,
    pub event_handler: Arc<EventHandler>,
    pub fetchers: Vec<Py<PyAny>>,
    #[pyo3(get, set)]
    pub domain: String,
}

#[pymethods]
impl Agent {
    #[new]
    fn new(base_class: Py<PyAny>, domain_name: Option<Py<PyString>>, features: Option<Vec<Py<PyAny>>>) -> Self {
        let domain_name = match domain_name {
            Some(domain_name) => domain_name.to_string(),
            None => {
                Python::with_gil(|py| {
                    let domain_name = base_class.getattr(py, "__domain__").unwrap();
                    domain_name.extract::<String>(py).unwrap()
                })
            }
        };
        let features = match features {
            Some(features) => features,
            None => {
                Python::with_gil(|py| {
                    let features = base_class.getattr(py, "__features__").unwrap();
                    features.extract::<Vec<Py<PyAny>>>(py).unwrap()
                })
            }
        };
        let agent = Agent {
            base_class,
            event_handler: Arc::new(EventHandler::new()),
            fetchers: features,
            domain: domain_name,
        };
        agent.__created__();
        agent
    }

    fn __created__(&self) {}

    fn emit(&self, event_type: EventType, data: Option<Py<PyAny>>) {
        let event_handler = Arc::clone(&self.event_handler);
        event_handler.emit(event_type, data);
    }

    fn add_event(&self, event_type: String, event: Event) {
        let event_handler = Arc::clone(&self.event_handler);
        match event_handler.get_editable_event_list(EventType::from_str(&event_type)) {
            Some(event_list) => event_list.register(event),
            None => panic!("Invalid event type"),
        }
    }
}


