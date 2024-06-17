
# --------------------------------------------------------------------------- # 

import subprocess

def linux_system_stats():
    cmd_linux = "sysstat -a"
    output = subprocess.check_output(cmd, shell=True)
    lines = output.decode().splitlines()

    cpu_usage = float(lines[0].strip())
    memory_usage = int(lines[1].strip().split(" ")[-2])
    disk_usage = int(lines[2].strip().split(" ")[-2])
    network_usage = float(lines[3].strip())

    return cpu_usage, memory_usage, disk_usage, network_usage

