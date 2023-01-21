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
    pub fn emit(&self, event_type: String) -> PyResult<()> {
        debug!("Emitting event from EventTypeHandler: {:?} {:?}", event_type,self.handlers);
        let mut jobs = Vec::new();
        for event in &self.handlers {
            let event = Arc::clone(&event);
            let job = async_std::task::spawn(async move {
                Python::with_gil(|py| {
                    let event = event.lock().unwrap();
                    let handler = event.handler.clone_ref(py);
                    let _ = handler.call0(py);
                });
            });
            jobs.push(job);
        }

        async_std::task::spawn(async move {
            for job in jobs {
                job.await;
            }
        });
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
    pub events: HashMap<String, RwLock<EventTypeHandler>>,
}

impl EventManager {
    pub fn register_event_handler(&mut self, name: String, handler: Py<PyAny>) -> PyResult<()> {
        debug!("Registering event {:?}, {:?}",name, handler);
        let event = Event {
            name: name.clone(),
            handler,
        };
        let event_type_handler = self.events.get(&name);
        match event_type_handler {
            Some(event_type_handler) => {
                let event_type_handler = event_type_handler.write().unwrap();
                event_type_handler.add_handler(event);
            }
            None => {
                let event_type_handler = EventTypeHandler {
                    name: name.clone(),
                    handlers: vec![Arc::new(Mutex::new(event))],
                };
                self.events.insert(name, RwLock::new(event_type_handler));
            }
        }
        Ok(())
    }

    pub fn emit(&self, event: String) -> PyResult<()> {
        debug!("Emitting event from EventManager : {:?}", event);
        let event = event.clone();
        let event_type_handler = self.events.get(&event);

        match event_type_handler {
            Some(event_type_handler) => {
                let event_type_handler = event_type_handler.read().unwrap();
                event_type_handler.emit(event)
            }
            None => {
                debug!("No event type handler found for event: {:?}", event);
                Ok(())
            }
        }
    }
}