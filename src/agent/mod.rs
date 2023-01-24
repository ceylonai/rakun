mod event;
mod message;
mod driver;

use std::sync::{Arc, Mutex};
use log::{debug, error};
use pyo3::{prelude::*};
use crate::agent::driver::{DriverManager};
use crate::agent::event::{EventManager};
use crate::agent::message::Message;


#[derive(Debug, Clone)]
#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    pub domain: String,
    pub event_manager: Arc<Mutex<EventManager>>,
    pub driver: Arc<DriverManager>,
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
            driver: Arc::new(DriverManager::new()),
        }
    }

    fn register_event_handler<'a>(&'a self, _py: Python<'a>, name: String, handler: Py<PyAny>) -> PyResult<&'a PyAny> {
        debug!("Registering event {:?}, {:?}",name, handler);
        let event_manager = self.event_manager.clone();
        let rx = async_std::task::spawn(async move {
            let mut event_manager = event_manager.lock().unwrap();
            match event_manager.register_event_handler(name, handler) {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
        });
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            match rx.await {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
        })
    }


    pub fn start<'a>(&'a self, _py: Python<'a>) -> PyResult<&'a PyAny> {
        debug!("Starting agent: {:?}", self.domain);
        let event_manager = Arc::clone(&self.event_manager);
        let rx = async_std::task::spawn(async move {
            let event_manager = event_manager.lock().unwrap();
            match event_manager.emit("after_start".to_string(), None) {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
        });

        let event_manager = Arc::clone(&self.event_manager);
        let driver = Arc::clone(&self.driver);
        let rx_message_handler = async_std::task::spawn(async move {
            while let Ok(msg) = driver.recv().await {
                debug!("Message received: {:?}", msg);
                event_manager.lock().unwrap().emit("on_message".to_string(), Option::from(msg.data)).unwrap();
            }
        });

        pyo3_asyncio::async_std::future_into_py(_py, async move {
            rx_message_handler.await;
            match rx.await {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
            // Ok(Python::with_gil(|py| py.None()))
        })
    }

    pub fn send<'a>(&'a self, _py: Python<'a>, data: Py<PyAny>) -> PyResult<&'a PyAny> {
        let driver = Arc::clone(&self.driver);
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            let id = uuid::Uuid::new_v4().to_string();
            let message = Message {
                id: id.clone(),
                data,
            };
            match driver.send(message).await {
                Ok(_) => {
                    debug!("Message:{} sent successfully", id.clone())
                }
                Err(e) => {
                    error!("Error sending message: {:?}", e)
                }
            }
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    pub fn exit<'a>(&'a self, _py: Python<'a>) -> PyResult<&'a PyAny> {
        debug!("Stopping agent: {:?}", self.domain);
        let event_manager = Arc::clone(&self.event_manager);
        let rx = async_std::task::spawn(async move {
            let event_manager = event_manager.lock().unwrap();
            match event_manager.emit("before_stop".to_string(), None) {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
        });
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            match rx.await {
                Ok(_) => Ok(Python::with_gil(|py| py.None())),
                Err(e) => Err(e),
            }
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Agent: {}", self.domain))
    }
}
