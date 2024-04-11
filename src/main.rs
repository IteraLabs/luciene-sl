
// Simply print values on console

use nvml_wrapper::enum_wrappers::device::{TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{cuda_driver_version_major, cuda_driver_version_minor, Nvml};
use pretty_bytes::converter::convert;

fn main() -> Result<(), NvmlError> {
    
    // Initialize client
    let nvml = Nvml::init()?;
    
    // Get the version of the cuda driver
    let cuda_version = nvml.sys_cuda_driver_version()?;

    // Index the first detected device
    let device = nvml.device_by_index(0)?;
    
    // Some metrics to show
    let name = device.name()?;
    let cuda_cores = device.num_cores()?; 
    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let mem_info = convert(device.memory_info()?.used as _);

    println!("Device Name: {name}, has {cuda_cores} cuda cores and memory of {memory_info}",
             name = name, cuda_cores = cuda_cores, memory_info = mem_info);

    println!("And it is running at {} temperature, with Cuda version {}",
             temperature, cuda_version);

    println!(
        "System CUDA version: {}.{}",
        cuda_driver_version_major(cuda_version),
        cuda_driver_version_minor(cuda_version)
    );

    Ok(())
}


