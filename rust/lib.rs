// lib.rs : Final script where to place wrapped functionality

use pyo3::prelude::*;
use std::env;

use pyo3::exceptions::PyRuntimeError;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use pretty_bytes::converter::convert;

#[pyfunction]
fn device_info() -> PyResult<String>{
// Initialize NVML
    let nvml = Nvml::init().map_err(|e| PyRuntimeError::new_err(format!("NVML Init Error: {}", e)))?;

    // Get the version of the cuda driver
    let cuda_version = nvml.sys_cuda_driver_version().map_err(|e| PyRuntimeError::new_err(format!("CUDA Version Error: {}", e)))?;

    // Index the first detected device
    let device = nvml.device_by_index(0).map_err(|e| PyRuntimeError::new_err(format!("Device Index Error: {}", e)))?;

    // Get general device information
    let name = device.name().map_err(|e| PyRuntimeError::new_err(format!("Device Name Error: {}", e)))?;

    Ok(name)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// Substract two numbers and returns the result as a string
#[pyfunction]
fn sub_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}


/// A Python module implemented in Rust. 
/// The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, 
/// else Python will not be able to import the module.

#[pymodule]
fn _lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sub_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(device_info, m)?)?;
    Ok(())
}
