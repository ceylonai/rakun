mod event;

use std::sync::{Arc, Mutex};
use log::{debug};
use pyo3::{prelude::*};
use crate::agent::event::{EventManager};


#[derive(Debug, Clone)]
#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    pub domain: String,
    pub event_manager: Arc<Mutex<EventManager>>,
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
            event_manager: Arc::new(Mutex::new(EventManager::default())),
        }
    }

    fn register_event_handler<'a>(&'a self, _py: Python<'a>, name: String, handler: Py<PyAny>) -> PyResult<&'a PyAny> {
        debug!("Registering event {:?}, {:?}",name, handler);
        let event_manager = self.event_manager.clone();
        let rx = async_std::task::spawn(async move {
            let event_manager = event_manager.lock().unwrap();
            event_manager.register_event_handler(name, handler)
        });
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            rx.await.expect("TODO: panic message");
            Ok(Python::with_gil(|py| py.None()))
        })
    }


    pub fn start<'a>(&'a self, _py: Python<'a>) -> PyResult<&'a PyAny> {
        debug!("Starting agent: {:?}", self.domain);
        let event_manager = self.event_manager.clone();
        let rx = async_std::task::spawn(async move {
            let event_manager = event_manager.lock().unwrap();
            Python::with_gil(|py| {
                event_manager.emit(py, "after_start".to_string()).unwrap();
            });
        });
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            rx.await;
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Agent: {}", self.domain))
    }
}

