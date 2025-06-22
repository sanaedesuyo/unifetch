use sysinfo::System;
use wmi::WMIError;
use crate::system_info::grabber::info_grab::Grabber;
use crate::system::component::Component;

pub enum ComponentType {
    Cpu,
    Gpu,
    Disk,
    Memory,
    OS,
}

pub fn grab(sys: &mut System, component_type: &ComponentType) -> Result<Vec<Box<dyn Component>>, WMIError> {
    sys.refresh_all();

    match component_type {
        ComponentType::Cpu => info_grab::CpuGrabber::grab(&sys),
        ComponentType::Gpu => info_grab::GpuGrabber::grab(&sys),
        ComponentType::Disk => info_grab::DiskGrabber::grab(&sys),
        ComponentType::Memory => info_grab::MemoryGrabber::grab(&sys),
        ComponentType::OS => info_grab::OSGrabber::grab(&sys),
    }
}

pub mod info_grab {
    use std::process::{Command};
    use sysinfo::{System, Disks};
    use crate::system::component::Component;
    use crate::system::cpu::CpuInfo;
    use crate::system::gpu::GpuInfo;
    use crate::system::disk::DiskInfo;

    #[cfg(target_os = "windows")]
    use wmi::{WMIError};
    use crate::system::memory::MemoryInfo;
    use crate::system::os::OSInfo;
    use crate::system_info::nvidia_grabber::NvidiaGrabber;

    pub trait Grabber {
        fn grab(sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError>;
    }

    pub struct CpuGrabber;
    impl Grabber for CpuGrabber {
        fn grab(sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
            let mut cpu_list = Vec::new();
            let cpu = sys.cpus().first().unwrap();

            cpu_list.push(Box::new(CpuInfo::new(
                cpu.brand().to_string(),
                sys.cpus().len(),
                cpu.cpu_usage(),
                sys.cpus().iter().map(|c| c.frequency()).sum::<u64>(),
            )) as Box<dyn Component>);

            Ok(cpu_list)
        }
    }

    pub struct GpuGrabber;
    impl Grabber for GpuGrabber {
        fn grab(_sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
            let mut gpu_list = Vec::new();

            #[cfg(target_os = "windows")] {
                let output = Command::new("wmic")
                    .args([
                        "path",
                        "win32_VideoController",
                        "get",
                        "Name,DriverVersion,CurrentHorizontalResolution,CurrentVerticalResolution,Status,AdapterRAM",
                        "/format:csv",
                    ])
                    .output()
                    .ok();

                let output = match output {
                    None => { return Err(WMIError::SerdeError("wmic returned empty result for request.".into())) }
                    Some(inner) => { inner }
                };

                let output = String::from_utf8(output.stdout).ok();
                let output = match output {
                    None => { return Err(WMIError::SerdeError("failed to convert wmic query result to String object.".into())) }
                    Some(inner) => { inner }
                };

                // Explain result from wmic command, pack into GpuInfo struct.
                let output = output.trim().split("\r\n").map(|s| s.to_string()).collect::<Vec<String>>();

                // Header of output result
                let header = output[0].split(',').map(|s| s.to_string()).collect::<Vec<String>>();
                for gpu_index in 1..output.len() {
                    let (mut name, mut driver, mut adapter_ram, mut status, mut horizontal_resolution, mut vertical_resolution) = (
                            String::new(),
                            String::new(),
                            0u64,
                            String::new(),
                            0u16,
                            0u16,
                        );
                    let data_row = &output[gpu_index];

                    let data_table = data_row.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
                    let mut is_nvidia_gpu = false;
                    for (index, property_name) in header.iter().enumerate() {
                        match property_name.as_str().trim() {
                            "Name" => {
                                name = String::from(&data_table[index]);

                                if name.contains("NVIDIA") {
                                    let n_gpu_info = NvidiaGrabber::grab(_sys).expect("failed to analysis NVIDIA GPU");
                                    is_nvidia_gpu = true;
                                    gpu_list.extend(n_gpu_info);
                                    break;
                                }
                            },
                            "DriverVersion" => { driver = String::from(&data_table[index]); },
                            "AdapterRAM" => { adapter_ram = u64::from_str_radix(&data_table[index], 10).unwrap_or(0) ; },
                            "Status" => { status = String::from(&data_table[index]); },
                            "CurrentHorizontalResolution" => { horizontal_resolution = u16::from_str_radix(&data_table[index], 10).unwrap_or(0); },
                            "CurrentVerticalResolution" => { vertical_resolution = u16::from_str_radix(&data_table[index], 10).unwrap_or(0); },

                            _ => {},
                        }
                    }

                    if is_nvidia_gpu {
                        continue;
                    }

                    // pack properties into struct
                    gpu_list.push(Box::new(GpuInfo::new(
                        name, driver, adapter_ram, status, horizontal_resolution, vertical_resolution
                    )) as Box<dyn Component>);
                }
            }

            Ok(gpu_list)
        }
    }

    pub struct DiskGrabber;
    impl Grabber for DiskGrabber {
        fn grab(_sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
            let mut disk_info = Vec::new();

            let disks = Disks::new_with_refreshed_list();

            for disk in disks.list() {
                let info = DiskInfo::new(
                    disk.name().to_str().unwrap().to_string(),
                    disk.file_system().to_str().unwrap().to_string(),
                    disk.total_space(),
                    disk.available_space(),
                    disk.kind().to_string(),
                );

                disk_info.push(Box::new(info) as Box<dyn Component>);
            }

            Ok(disk_info)
        }
    }

    pub struct MemoryGrabber;
    impl Grabber for MemoryGrabber {
        fn grab(sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
            let (used, total) = (sys.used_memory(), sys.total_memory());

            let memory_info = vec![Box::new(
                MemoryInfo::new(
                    total,
                    total - used,
                )
            ) as Box<dyn Component>];

            Ok(memory_info)
        }
    }

    pub struct OSGrabber;
    impl Grabber for OSGrabber {
        fn grab(_sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
            let mut os_info = Vec::new();

            let name = match System::long_os_version() {
                Some(name) => name,
                None => { return Err(WMIError::SerdeError("failed to detect OS version.".into())) }
            };

            let host_name = match System::host_name() {
                Some(name) => name,
                None => { return Err(WMIError::SerdeError("failed to detect Host name.".into())) }
            };

            os_info.push(Box::new(OSInfo::new(
                name, host_name
            )) as Box<dyn Component>);

            Ok(os_info)
        }
    }
}