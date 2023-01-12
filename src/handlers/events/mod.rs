use std::collections::HashMap;
use std::sync::Arc;
use pyo3::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EventInner {
    BeforeStart,
    AfterStart,
    BeforeStop,
    AfterStop,
    Message,
}

#[derive(Clone)]
pub struct EventType(EventInner);

impl EventType {
    const BEFORE_START: EventType = EventType(EventInner::BeforeStart);
    const AFTER_START: EventType = EventType(EventInner::AfterStart);
    const BEFORE_STOP: EventType = EventType(EventInner::BeforeStop);
    const AFTER_STOP: EventType = EventType(EventInner::AfterStop);
    const MESSAGE: EventType = EventType(EventInner::Message);
}

#[derive(Clone)]
#[pyclass]
pub struct Event {
    pub event_type: String,
    pub filter: Option<String>,
    pub method: Py<PyAny>,
}

#[pymethods]
impl Event {
    #[new]
    pub fn new(event_type: String, method: Py<PyAny>) -> Self {
        Event { event_type, method, filter: None }
    }
}

// type RouteMap = RwLock<MatchItRouter<Response>>;

pub struct EventHandler {
    events: HashMap<String, Vec<Event>>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            events: HashMap::new(),
        }
    }

    pub fn add_event(&mut self, event_type: String, event: Event) {
        self.events.entry(event_type).or_default().push(event);
    }
}