use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use log::info;
use pyo3::ffi::Py_None;
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
            "after_agent_start" => EventType(EventInner::AfterAgentStart),
            "before_agent_stop" => EventType(EventInner::BeforeAgentStop),
            "message" => EventType(EventInner::Message),
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
    pub const AFTER_AGENT_START: EventType = EventType(EventInner::AfterAgentStart);
    pub const BEFORE_AGENT_STOP: EventType = EventType(EventInner::BeforeAgentStop);
    pub const MESSAGE: EventType = EventType(EventInner::Message);
    pub const ON_EVENT: EventType = EventType(EventInner::OnEvent);
    pub const FOREVER: EventType = EventType(EventInner::Forever);
    pub const PERIODIC: EventType = EventType(EventInner::Periodic);
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

    pub fn action<'a>(&'a self, py: Python<'a>, args: Option<Py<PyAny>>) -> PyResult<&'a PyAny> {
        info!("Event action {:?} {:?} {:?}", self.event_type, self.method.to_string(), self.filter);
        let method = self.method.as_ref(py);
        let meth = method.call1((args, )).unwrap();
        let meth = pyo3_asyncio::async_std::into_future(meth);
        pyo3_asyncio::async_std::future_into_py(py, async move {
            info!("Event action future");
            match meth {
                Ok(meth) => {
                    info!("Event action future meth");
                    let meth = meth.await;
                    info!("Event action future meth {:?}", meth);
                    match meth {
                        Ok(meth) => {
                            info!("Event action future meth ok {:?}", meth);
                            Ok(meth)
                        }
                        Err(e) => {
                            info!("Event action future meth err {:?}", e);
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    info!("Event action future err {:?}", e);
                    Err(e)
                }
            }
        })
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
        events.insert(EventType::AFTER_AGENT_START.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::BEFORE_AGENT_STOP.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
        events.insert(EventType::MESSAGE.as_string(), EventMap { event_map: RwLock::new(Vec::new()) });
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
                    Python::with_gil(|py| {
                        event.action(py, _data).unwrap();
                    });
                }
            }
            None => {
                info!("No event found for event type: {}", event_type.as_str());
            }
        }
    }
}