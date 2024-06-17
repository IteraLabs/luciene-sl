
# --------------------------------------------------------------------------- # 
import os
path = os.environ['PATH']
local_folder = os.getcwd('.')
if local_folder not in path:
    path += os.pathsep + local_folder
os.environ['PATH'] = path

# -- Only export public parts of the binary
from _lib import sum_as_string as lucien_sum 
from _lib import sub_as_string as lucien_sub
from _lib import device_info

__all__ = ["lucien_sum", "lucien_sub", "device_info"]
