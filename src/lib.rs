mod agent;

use log::debug;
use pyo3::prelude::*;
use crate::agent::Agent;

#[pyfunction]
fn start_agents(py: Python, agents: Vec<Py<Agent>>) -> PyResult<&PyAny> {
    for agent in agents {
        let agent = agent.as_ref(py);
        debug!("Starting agent: {:?}", agent);
    }
    pyo3_asyncio::async_std::future_into_py(py, async move {
        debug!("Starting async");
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn rakun(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(start_agents, m)?)?;
    m.add_class::<Agent>()?;
    // m.add_class::<Event>()?;
    // m.add_class::<Driver>()?;
    Ok(())
}