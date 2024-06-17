# ! /usr/bin/env python3

import os
import subprocess
import torch
import json

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def ioreg_gpu_arch_info():

    cmd_ioreg = "ioreg -l"
    sc_io = "IONameMatched"
    sc_cc = "gpu-core-count"
    sc_cv = "GPUConfigurationVariable"
 
    cmd_ioreg_io = cmd_ioreg + " " + sc_io 
    cmd_ioreg_cc = cmd_ioreg + " " + sc_cc
    cmd_ioreg_cv = cmd_ioreg + " " + sc_cv

    pass

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def system_profiler_gpu_arch_info():
    """
    """ 
    
    sp_ddt = "SPDisplaysDataType" 
    cmd_system_profiler = "system_profiler -detailLevel mini -json"
    cmd_r = subprocess.check_output(cmd_system_profiler + " " + sp_ddt,
                                    shell=True,
                                    text=True)
    cmd_r = json.loads(cmd_r)

    _system_profiler_gpu_arch = {
        'model': cmd_r[sp_ddt][0]['sppci_model'],
        'cores': cmd_r[sp_ddt][0]['sppci_cores'],
        'mtlgpufamilysupport': cmd_r[sp_ddt][0]['spdisplays_mtlgpufamilysupport']
    }
    
    return _system_profiler_gpu_arch

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def _torch_gpu_arch_info():
    """
    """

    gpu_arch = {'mps': torch.backends.mps.is_available(),
                'cudnn': torch.backends.cudnn.is_available(),
                }

    return gpu_arch

# ------------------------------------------------------------------------------------ #
# ------------------------------------------------------------------------------------ #

def gpu_arch(verbose: bool = True):
    """
    Describe the architecture characteristics of the GPU

    Args:
        verbose (bool): Indication to print results in console

    Raises:
        ValueError: When an error was encountered during
        the execution of torch.backends.gpu

    Yields:
        None

    Returns:
        dict[str, str]: With the metrics as keys and their returned values as values

    Example:
        >>> gpu_arch(verbose=True)

    """
   
    try:
        gpu_arch = _torch_gpu_arch_info()
    
        if gpu_arch['mps'] is True:

            mps_device = torch.device("mps")
            tensor_device = torch.ones(1, device=mps_device)
            
            if verbose:

                print(f"\n -- GPU architecture detection -- \n")
                print(" MPS supported device found \n" + \
                      f"\n torch.backends.mps.is_available() is " + \
                       "{gpu_arch['mps']} \n" + \
                      f"\n >>> mps_device = torch.device('mps')" + \
                      f"\n >>> torch.ones(1, device=mps_device)" + \
                      f"\n {tensor_device}")

        elif gpu_arch['cudnn'] is True:

            cuda_device = torch.device("cudnn")
            tensor_device = torch.ones(1, device=cuda_device)
            
            if verbose:
                print(" CUDA supported device found \n" + \
                      f"\n torch.backends.cudnn.is_available() is " + \
                       "{gpu_arch['cudnn']} \n" + \
                      f"\n >>> cuda_device = torch.device('cudnn')" + \
                      f"\n >>> torch.ones(1, device=cuda_device)" + \
                      f"\n {tensor_device}")

    except:
        raise ValueError(f"torch.backends.gpu failed")
    
    return gpu_arch 
    
