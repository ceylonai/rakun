use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::info;

use pyo3::prelude::*;
use pyo3::types::{PyDictItems, PyString};
use crate::drivers::Driver;

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
    fn __new__(base_class: Py<PyAny>, domain_name: Option<Py<PyString>>,
               features: Option<Vec<Py<PyAny>>>,
               events: Option<HashMap<String, Vec<Event>>>) -> Self {
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

        let event_handler = EventHandler::new();
        if let Some(events) = events {
            for (event_type, handlers) in events {
                for handler in handlers {
                    event_handler.get_editable_event_list(EventType::from_str(&event_type))
                        .unwrap().register(handler);
                }
            }
        }
        Agent {
            base_class,
            event_handler: Arc::new(event_handler),
            features,
            domain: domain_name,
        }
    }

    fn start<'a>(&'a self, py: Python<'a>, driver: Option<Driver>) -> PyResult<&'a PyAny> {
        Python::with_gil(|py| {
            let asyncio = py.import("asyncio").unwrap();

            let event_loop = asyncio.call_method0("new_event_loop").unwrap();
            asyncio
                .call_method1("set_event_loop", (event_loop, ))
                .unwrap();
            self.emit(EventType::AFTER_AGENT_START, None);

            info!("Starting agent with driver");
        });

        pyo3_asyncio::async_std::future_into_py(py, async move {
            Ok(Python::with_gil(|py| "ok".to_object(py)))
        })
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


