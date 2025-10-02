use pyo3::{exceptions::PyRuntimeError, prelude::*, wrap_pyfunction};
use std::process::Command;

#[pyfunction]
fn start_client(config_path: String) -> PyResult<()> {
    let output = Command::new("rathole")
        .arg("-c")
        .arg(&config_path)
        .output()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to start rathole client: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(PyRuntimeError::new_err(format!("Rathole client failed: {}", error_msg)));
    }
    
    Ok(())
}

#[pyfunction]
fn start_server(config_path: String) -> PyResult<()> {
    let output = Command::new("rathole")
        .arg("-s")
        .arg(&config_path)
        .output()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to start rathole server: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(PyRuntimeError::new_err(format!("Rathole server failed: {}", error_msg)));
    }
    
    Ok(())
}

#[pyfunction]
fn version() -> PyResult<String> {
    let output = Command::new("rathole")
        .arg("--version")
        .output()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to get rathole version: {}", e)))?;
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(PyRuntimeError::new_err(format!("Failed to get version: {}", error_msg)));
    }
    
    let version = String::from_utf8_lossy(&output.stdout);
    Ok(version.trim().to_string())
}

#[pymodule]
fn pyrathole(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_client, m)?)?;
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
