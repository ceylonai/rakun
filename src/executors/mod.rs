use log::info;
use pyo3::{Py, PyAny, PyResult, Python};
use pyo3::ffi::PyObject;

pub fn execute_method<'a>(
    method: Py<PyAny>,
    py: Python<'a>,
    args: Option<Py<PyAny>>,
) -> PyResult<&'a PyAny> {
    let method = method.as_ref(py);
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