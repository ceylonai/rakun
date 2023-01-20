use std::collections::HashMap;
use std::sync::RwLock;
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
    pub handlers: Vec<Event>,
}

impl EventTypeHandler {
    pub fn emit(&self, py: Python, event: &String) -> PyResult<()> {
        debug!("Emitting event from EventTypeHandler: {:?} {:?}", event,self.handlers);
        for handler in &self.handlers {
            handler.handler.call0(py)?;
        }
        Ok(())
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
                event_type_handler.handlers.push(event);
            }
            None => {
                let mut event_type_handler = EventTypeHandler {
                    name: name.clone(),
                    handlers: Vec::new(),
                };
                event_type_handler.handlers.push(event);
                events.insert(name, event_type_handler);
            }
        }
        Ok(())
    }

    pub fn emit(&self, py: Python, event: String) -> PyResult<()> {
        debug!("Emitting event from EventManager : {:?}", event);
        let events = self.events.read().unwrap();
        let event_type_handler = events.get(&event);
        match event_type_handler {
            Some(event_type_handler) => {
                event_type_handler.emit(py, &event)
            }
            None => {
                debug!("No event type handler found for event: {:?}", event);
                Ok(())
            }
        }
    }
}