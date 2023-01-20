use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use log::debug;
use pyo3::{prelude::*};

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub handler: Py<PyAny>,
}

#[derive(Debug, Clone)]
pub struct EventTypeHandler {
    pub name: String,
    pub handlers: Vec<Arc<Mutex<Event>>>,
}

impl EventTypeHandler {
    pub fn emit(&self, event_type: &String) -> PyResult<()> {
        debug!("Emitting event from EventTypeHandler: {:?} {:?}", event_type,self.handlers);
        for event in &self.handlers {
            let event = event.lock().unwrap();
            let output = Python::with_gil(|py| {
                let function_output = event.handler.as_ref(py).call0().unwrap();
                pyo3_asyncio::async_std::into_future(function_output)
            }).unwrap();

            async_std::task::spawn(async move {
                let output = output.await;
                Python::with_gil(|py| {
                    let output = output.unwrap();
                    let output = output.as_ref(py);
                    let output = output.to_string();
                    debug!("Event output: {:?}", output);
                });
            });


            // let _ = Python::with_gil(|py| {
            //     event.handler.call0(py)
            // });
        }
        Ok(())
    }

    pub fn add_handler(&self, event: Event) {
        debug!("Adding handler to EventTypeHandler: {:?}", event);
        let mut handlers = self.handlers.clone();
        handlers.push(Arc::new(Mutex::new(event)));
    }
}

#[derive(Debug, Default)]
pub struct EventManager {
    pub events: RwLock<HashMap<String, EventTypeHandler>>,
}

impl EventManager {
    pub fn register_event_handler(&self, name: String, handler: Py<PyAny>) -> PyResult<()> {
        debug!("Registering event {:?}, {:?}",name, handler);
        let mut events = self.events.write().unwrap();
        let event = Event {
            name: name.clone(),
            handler,
        };
        match events.get_mut(&name) {
            Some(event_type_handler) => {
                event_type_handler.add_handler(event);
            }
            None => {
                let event_type_handler = EventTypeHandler {
                    name: name.clone(),
                    handlers: vec![Arc::new(Mutex::new(event))],
                };
                events.insert(name, event_type_handler);
            }
        }
        Ok(())
    }

    pub fn emit(&self, event: String) -> PyResult<()> {
        debug!("Emitting event from EventManager : {:?}", event);
        let events = self.events.read().unwrap();
        let event_type_handler = events.get(&event);
        match event_type_handler {
            Some(event_type_handler) => {
                event_type_handler.emit(&event)
            }
            None => {
                debug!("No event type handler found for event: {:?}", event);
                Ok(())
            }
        }
    }
}