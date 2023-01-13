mod agent;
mod handlers;

use agent::Agent;
use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn rakun(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Agent>()?;
    Ok(())
}