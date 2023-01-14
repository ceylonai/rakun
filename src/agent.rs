use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::handlers::events::{Event, EventHandler, EventType};

#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    pub base_class: PyObject,
    pub event_handler: Arc<EventHandler>,
    #[pyo3(get)]
    pub features: Vec<Py<PyAny>>,
    #[pyo3(get)]
    pub domain: String,
}

#[pymethods]
impl Agent {
    #[new]
    fn __new__(base_class: Py<PyAny>, domain_name: Option<Py<PyString>>, features: Option<Vec<Py<PyAny>>>) -> Self {
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

        let base_class = Python::with_gil(|py| {
            let base_class = base_class.as_ref(py);
            let base_class = base_class.call0().unwrap();
            base_class.to_object(py)
        });

        Agent {
            base_class,
            event_handler: Arc::new(EventHandler::new()),
            features,
            domain: domain_name,
        }
    }


    /**
     * Emit events
     */
    fn emit(&self, event_type: EventType, data: Option<Py<PyAny>>) {
        let event_handler = Arc::clone(&self.event_handler);
        event_handler.emit(event_type, data);
    }

    fn add_event(&self, event_type: String, event: Event) {
        let event_handler = Arc::clone(&self.event_handler);
        match event_handler.get_editable_event_list(EventType::from_str(&event_type)) {
            Some(event_list) => event_list.register(event),
            None => panic!("Invalid event type {}", event_type),
        }
    }
}


