
import subprocess


def macos_system_stats():

    macos_cmd = "sysctl"

    cpu_cmd = "sysctl -n hw.cpu_usage"
    cpu_usage = float(subprocess.check_output(cpu_cmd, shell=True))

    memory_cmd = "sysctl -n vm.total_memory"
    memory_usage = int(subprocess.check_output(cmd, shell=True))

    disk_cmd = "sysctl -n hw.disk_total"
    disk_usage = int(subprocess.check_output(cmd, shell=True))
    
    network_cmd = "sysctl -n net.if.transmit_bytes"
    network_usage = float(subprocess.check_output(cmd, shell=True))

    return cpu_usage, memory_usage, disk_usage, network_usage

def linux_system_stats():
    cmd_linux = "sysstat -a"
    output = subprocess.check_output(cmd, shell=True)
    lines = output.decode().splitlines()

    cpu_usage = float(lines[0].strip())
    memory_usage = int(lines[1].strip().split(" ")[-2])
    disk_usage = int(lines[2].strip().split(" ")[-2])
    network_usage = float(lines[3].strip())

    return cpu_usage, memory_usage, disk_usage, network_usage

macos_system_stats()
