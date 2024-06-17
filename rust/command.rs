use pyo3::prelude::*;
use nvml_wrapper::NVML;
use std::io::{self, Write};
use std::net::TcpStream;

// Command trait
trait Command {
    fn execute(&self) -> PyResult<f32>;
}

// TODO: An initialization function for NVML

// Concrete Command for GPU utilization
struct GpuUtilizationCommand;

impl Command for GpuUtilizationCommand {
    fn execute(&self) -> PyResult<f32> {
        
        // Abstract away this in a init_function()
        let nvml = NVML::init()?;
        
        // 
        let device = nvml.device_by_index(0)?;
        let utilization = device.utilization_rates()?;
        
        Ok(utilization.gpu as f32)
    }
}

// Concrete Command for network throughput (simplified example)
struct NetworkThroughputCommand;

impl Command for NetworkThroughputCommand {
    fn execute(&self) -> PyResult<f32> {
        let mut stream = TcpStream::connect("8.8.8.8:80")?;
        stream.write_all(b"GET / HTTP/1.1\r\nHost: google.com\r\n\r\n")?;
        // Simplification: Assume fixed throughput value
        Ok(100.0)  // Dummy value for throughput
    }
}

#[pyfunction]
fn execute_command(command: &dyn Command) -> PyResult<f32> {
    command.execute()
}

// #[pymodule]
// fn metrics_collector(py: Python, m: &PyModule) -> PyResult<()> {
//      m.add_function(wrap_pyfunction!(execute_command, m)?)?;
//      Ok(())
//}

