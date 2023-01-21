mod agent;

use pyo3::prelude::*;
use crate::agent::Agent;

#[pyfunction]
fn start_agents(_py: Python, agents: Vec<Py<Agent>>) -> PyResult<&PyAny> {
    // let mut jobs = Vec::new();
    for agent in agents {
        let agent = agent.clone_ref(_py).extract::<Agent>(_py).unwrap();
        agent.start(_py).unwrap();
    }
    pyo3_asyncio::async_std::future_into_py(_py, async move {
        // for job in jobs {
        //     job.await;
        // }
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