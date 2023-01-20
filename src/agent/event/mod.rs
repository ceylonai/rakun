use std::collections::HashMap;
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
        debug!("Emitting event from Event type handler : {:?}", event);
        for handler in &self.handlers {
            handler.handler.call0(py)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventManager {
    pub events: HashMap<String, EventTypeHandler>,
}

impl EventManager {
    pub fn emit(&self, py: Python, event: String) -> PyResult<()> {
        debug!("Emitting event from EventManager : {:?}", event);
        debug!("EventManager registered events: {:?}", self.events);
        match self.events.get(&event) {
            Some(event_type) => event_type.emit(py, &event),
            None => Ok(()),
        }
    }
}