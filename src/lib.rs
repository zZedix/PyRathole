use pyo3::{exceptions::PyRuntimeError, prelude::*, wrap_pyfunction};

#[pyfunction]
fn start_client(config: String) -> PyResult<()> {
    rathole::start_client(&config)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
fn start_server(config: String) -> PyResult<()> {
    rathole::start_server(&config)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
fn stop() -> PyResult<()> {
    rathole::stop()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

#[pymodule]
fn pyrathole(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_client, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(stop, m)?)?;
    Ok(())
}
