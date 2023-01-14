use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use pyo3::prelude::*;

// forever, periodic, on_event, on_message, on_start, on_stop
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EventInner {
    BeforeAgentStart,
    AfterAgentStart,
    BeforeAgentStop,
    AfterAgentStop,
    Message,
    OnStart,
    OnStop,
    OnEvent,
    Forever,
    Periodic,
}

#[derive(Clone)]
#[pyclass]
pub struct EventType(EventInner);

impl EventType {
    pub fn from_str(event_type: &String) -> Self {
        match event_type.as_str() {
            "before_agent_start" => EventType(EventInner::BeforeAgentStart),
            "after_agent_start" => EventType(EventInner::AfterAgentStart),
            "before_agent_stop" => EventType(EventInner::BeforeAgentStop),
            "after_agent_stop" => EventType(EventInner::AfterAgentStop),
            "message" => EventType(EventInner::Message),
            "on_start" => EventType(EventInner::OnStart),
            "on_stop" => EventType(EventInner::OnStop),
            "on_event" => EventType(EventInner::OnEvent),
            "forever" => EventType(EventInner::Forever),
            "periodic" => EventType(EventInner::Periodic),
            _ => panic!("Invalid event type {}", event_type),
        }
    }

    pub fn as_str(&self) -> &str {
        match self.0 {
            EventInner::BeforeAgentStart => "before_agent_start",
            EventInner::AfterAgentStart => "after_agent_start",
            EventInner::BeforeAgentStop => "before_agent_stop",
            EventInner::AfterAgentStop => "after_agent_stop",
            EventInner::Message => "message",
            EventInner::OnStart => "on_start",
            EventInner::OnStop => "on_stop",
            EventInner::OnEvent => "on_event",
            EventInner::Forever => "forever",
            EventInner::Periodic => "periodic",
        }
    }

    pub fn as_string(&self) -> String {
        String::from(self.as_str())
    }
}

impl EventType {
    const BEFORE_AGENT_START: EventType = EventType(EventInner::BeforeAgentStart);
    const AFTER_AGENT_START: EventType = EventType(EventInner::AfterAgentStart);
    const BEFORE_AGENT_STOP: EventType = EventType(EventInner::BeforeAgentStop);
    const AFTER_AGENT_STOP: EventType = EventType(EventInner::AfterAgentStop);
    const MESSAGE: EventType = EventType(EventInner::Message);
    const ON_START: EventType = EventType(EventInner::OnStart);
    const ON_STOP: EventType = EventType(EventInner::OnStop);
    const ON_EVENT: EventType = EventType(EventInner::OnEvent);
    const FOREVER: EventType = EventType(EventInner::Forever);
    const PERIODIC: EventType = EventType(EventInner::Periodic);
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
        let mut events = HashMap::new();
        events.insert(EventType::BEFORE_AGENT_START.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::AFTER_AGENT_START.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::BEFORE_AGENT_STOP.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::AFTER_AGENT_STOP.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::MESSAGE.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::ON_START.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::ON_STOP.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::ON_EVENT.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::FOREVER.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::PERIODIC.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        Self { events }
    }

    pub fn get_editable_event_list(&self, event_type: EventType) -> Option<&EventMap> {
        self.events.get(event_type.as_str())
    }

    pub fn emit(&self, event_type: EventType, data: Option<Py<PyAny>>) {
        let event_list = self.get_editable_event_list(event_type.clone());
        match event_list {
            Some(event_list) => {
                let event_list = event_list.get_event_list();
                for event in event_list {
                    let _data = data.clone();
                    if Python::with_gil(|py| {
                        let method = event.method.as_ref(py);
                        let rep = method.call1((_data, )).unwrap();
                        pyo3_asyncio::async_std::into_future(rep)
                    }).is_ok() {}
                }
            }
            None => {
                println!("No event found for event type: {}", event_type.as_str());
            }
        }
    }
}