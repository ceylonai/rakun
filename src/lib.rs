mod agent;
mod handlers;
mod drivers;

use agent::Agent;
use pyo3::prelude::*;
use crate::drivers::Driver;
use crate::handlers::events::Event;


/// A Python module implemented in Rust.
#[pymodule]
fn rakun(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Agent>()?;
    m.add_class::<Event>()?;
    m.add_class::<Driver>()?;
    Ok(())
}