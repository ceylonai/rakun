use std::collections::HashMap;
use std::sync::{Arc};
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
    pub handlers: Vec<Arc<Event>>,
}

impl EventTypeHandler {
    pub fn emit(&self, event_type: String, data: Option<Py<PyAny>>) -> PyResult<()> {
        debug!("Emitting event from EventTypeHandler: {:?} {:?}", event_type,self.handlers);
        let mut jobs = Vec::new();
        for event in &self.handlers {
            let event = Arc::clone(&event);
            let data = data.clone();
            let job = async_std::task::spawn(async move {
                Python::with_gil(|py| {
                    let event = Arc::clone(&event);
                    let handler = event.handler.clone_ref(py);
                    let async_method = match data {
                        Some(data) => handler.call1(py, (data, )).unwrap(),
                        None => handler.call0(py).unwrap(),
                    };
                    let asyncio = py.import("asyncio").unwrap();
                    let event_loop = asyncio.call_method0("new_event_loop").unwrap();
                    asyncio.call_method1("set_event_loop", (event_loop, )).unwrap();
                    let event_loop_hdl = PyObject::from(event_loop);
                    event_loop_hdl.as_ref(py).call_method1("run_until_complete", (async_method, )).unwrap();
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
        handlers.push(Arc::new(event));
    }
}

#[derive(Debug, Default)]
pub struct EventManager {
    pub events: HashMap<String, EventTypeHandler>,
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
                // let event_type_handler = event_type_handler.unwrap();
                event_type_handler.add_handler(event);
            }
            None => {
                let event_type_handler = EventTypeHandler {
                    name: name.clone(),
                    handlers: vec![Arc::new(event)],
                };
                self.events.insert(name, event_type_handler);
            }
        }
        Ok(())
    }

    pub fn emit(&self, event: String, data: Option<Py<PyAny>>) -> PyResult<()> {
        debug!("Emitting event from EventManager : {:?}", event);
        let event = event.clone();
        let event_type_handler = self.events.get(&event);
        match event_type_handler {
            Some(event_type_handler) => {
                event_type_handler.emit(event, data)
            }
            None => {
                debug!("No event type handler found for event: {:?}", event);
                Ok(())
            }
        }
    }
}