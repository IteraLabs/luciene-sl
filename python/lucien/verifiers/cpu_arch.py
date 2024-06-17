# ! /usr/bin/env python3

import subprocess
import torch
import json

def system_profiler_cpu_arch_info():
    """
    Profiling CPU architecture.

    """

    sp_hdt = "SPHardwareDataType"
    cmd_system_profiler = "system_profiler -detailLevel mini -json"
    cmd_r = subprocess.check_output(cmd_system_profiler + " " + sp_hdt,
                                    shell=True,
                                    text=True)
    cmd_r = json.loads(cmd_r)
 
    _system_profiler_cpu_arch = {
        'chip_type': cmd_r[sp_hdt][0]['chip_type'],
        'os_loader_version': cmd_r[sp_hdt][0]['os_loader_version'],
        'number_processors': cmd_r[sp_hdt][0]['number_processors'],
        'physical_memory': cmd_r[sp_hdt][0]['physical_memory']
    }
    
    return _system_profiler_cpu_arch

# --------------------------------------------------------------------------- #
# --------------------------------------------------------------------------- #

def sysctl_cpu_arch_info():
    """
    """

    cmd_sysctl = "sysctl -n"
    sc_b = "machdep.cpu.brand_string"
    sc_c = "machdep.cpu.core_count"

    cmd_sysctl_b = cmd_sysctl + " " + sc_b
    cmd_sysctl_c = cmd_sysctl + " " + sc_c

    cmd_b = subprocess.check_output(cmd_sysctl_b,
                                    shell=True,
                                    text=True)

    # cmd_b = json.loads(cmd_b)

    cmd_c = subprocess.check_output(cmd_sysctl_c,
                                    shell=True,
                                    text=True)

    # cmd_c = json.loads(cmd_c)

    _sysctl_cpu_arch = {
        'machdep.cpu.brand_string': cmd_b.replace('\n', ''),
        'machdep.cpu.core_count' : cmd_c.replace('\n', ''),
    }
    

    return _sysctl_cpu_arch

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def _torch_cpu_arch_info():
    """
    Internal helper function
    """

    _arch = {'VSX': torch.backends.cpu.get_cpu_capability() == 'VSX',
             'Z Vector': torch.backends.cpu.get_cpu_capability() == 'Z Vector',
             'NO AVX': torch.backends.cpu.get_cpu_capability() == 'NO AVX',
             'AVX2':torch.backends.cpu.get_cpu_capability() == 'AVX2',
             'AVX512':torch.backends.cpu.get_cpu_capability() == 'AVX512',
            }
    
    return _arch

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def cpu_arch(verbose: bool = True) -> dict[str, str]:
    """
    Describe the architecture characteristics of the CPU

    Args:
        verbose (bool): indication to print or not the values in console

    Raises:
        ValueError: When an error was encountered during 
        the execution of torch.backends.cpu

    Yields:
        None

    Returns:
        dict: with metrics as keys and their returned values as the value.
    
    Example:
        >>> cpu_arch(verbose=True)
        
    """
    
    try:
        cpu_arch = _torch_cpu_arch_info()
        if verbose:
            print(f"\n -- CPU architecture detection (torch) -- \n")
            print("\n".join(f" {key}: {value}" for key, value in cpu_arch.items())) 
    except:
        raise ValueError(f"torch.backends.cpu failed")
    
    return cpu_arch 

