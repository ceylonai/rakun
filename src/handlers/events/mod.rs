use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
    pub fn from_str(event_type: &String) -> Self {
        match event_type.as_str() {
            "before_start" => EventType(EventInner::BeforeStart),
            "after_start" => EventType(EventInner::AfterStart),
            "before_stop" => EventType(EventInner::BeforeStop),
            "after_stop" => EventType(EventInner::AfterStop),
            "message" => EventType(EventInner::Message),
            _ => panic!("Invalid event type"),
        }
    }

    pub fn as_str(&self) -> &str {
        match self.0 {
            EventInner::BeforeStart => "before_start",
            EventInner::AfterStart => "after_start",
            EventInner::BeforeStop => "before_stop",
            EventInner::AfterStop => "after_stop",
            EventInner::Message => "message",
        }
    }
}

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

pub struct EventMap {
    pub event_map: RwLock<Vec<Event>>,
}

impl EventMap {
    pub fn register(&self, event: Event) {
        let mut event_map = self.event_map.write().unwrap();
        event_map.push(event);
    }
    pub fn get_event_list(&self) -> Vec<Event> {
        let event_map = self.event_map.read().unwrap();
        event_map.clone()
    }
}

pub struct EventHandler {
    events: HashMap<String, EventMap>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            events: HashMap::new(),
        }
    }

    pub fn get_editable_event_list(&self, event_type: EventType) -> Option<&EventMap> {
        match event_type {
            _ => self.events.get(event_type.as_str()),
        }
    }
}