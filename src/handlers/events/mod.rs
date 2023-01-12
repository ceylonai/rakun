use std::collections::HashMap;
use std::sync::Arc;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub enum EventType {
    BeforeStart,
    AfterStart,
    BeforeStop,
    AfterStop,
    Message,
}

#[pyclass]
pub struct Event {
    pub event_type: EventType,
    pub method: Py<PyAny>,
}

#[pymethods]
impl Event {
    #[new]
    pub fn new(event_type: EventType, method: Py<PyAny>) -> Self {
        Event { event_type, method }
    }
}

pub type EventHandler = HashMap<EventType, Vec<Event>>;