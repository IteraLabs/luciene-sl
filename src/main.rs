
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
    
    // General Device Information
    let name = device.name()?;
    let uuid = device.uuid()?;
    let board_id = device.board_id()?;
    let cuda_cores = device.num_cores()?; 
    let mem_info = convert(device.memory_info()?.used as _);

    // Operational State Information
    let temperature = device.temperature(TemperatureSensor::Gpu)?;
    let power_usage = device.power_usage()?;
    
    // Hardware context for data transfer 
    let pcie_link_gen = device.current_pcie_link_gen()?;
    let pcie_link_speed = device
        .pcie_link_speed()?;

    let pcie_link_width = device.current_pcie_link_width()?;
    let pcie_max_link_gen = device.max_pcie_link_gen()?;

    println!("\nGeneral Device Information: \n
             Device Name: {name}
             Device UUID: {uuid}
             Board ID : {board_id}
             Cuda Cores: {cuda_cores}
             Memory: {memory_info}",
             name = name, 
             uuid = uuid,
             board_id = board_id,
             cuda_cores = cuda_cores,
             memory_info = mem_info);

    println!("\nOperational State Information: \n
             Temperature: {temperature}
             Power Usage: {power_usage}",
             temperature = temperature, 
             power_usage = power_usage);

    println!("\nHardware Context: Data Transfer: \n
             PCIe Link Generation : {pcie_link_gen}
             PCIe Link Speed: {pcie_link_speed}
             PCIe Link Bandwitdh: {pcie_link_width}
             PCIe Max Link Generation: {pcie_max_link_gen}", 
             pcie_link_gen = pcie_link_gen, 
             pcie_link_speed = pcie_link_speed, 
             pcie_link_width = pcie_link_width, 
             pcie_max_link_gen = pcie_max_link_gen);

    println!("\nSystem CUDA version: {}.{}",
        cuda_driver_version_major(cuda_version),
        cuda_driver_version_minor(cuda_version)
    );

    Ok(())
}


