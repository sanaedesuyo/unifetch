use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;
use sysinfo::System;
use wmi::WMIError;
use crate::system::component::Component;
use crate::system::nvidia_gpu::NvidiaGpuInfo;
use crate::system_info::grabber::info_grab::Grabber;

pub struct NvidiaGrabber;

impl Grabber for NvidiaGrabber {
    fn grab(_sys: &System) -> Result<Vec<Box<dyn Component>>, WMIError> {
        let mut nvidia_gpu_info = Vec::new();

        match Nvml::init() {
            Ok(nvml) => {
                match nvml.device_count() {
                    Ok(device_count) => {
                        for i in 0..device_count {
                            #[allow(unused_variables)]
                            let mut nvidia_gpu = NvidiaGpuInfo::default();

                            if let Ok(device) = nvml.device_by_index(i) {
                                nvidia_gpu.name = device.name().unwrap_or(String::from("Unknown"));
                                nvidia_gpu.driver_version = nvml.sys_driver_version().unwrap_or(String::from("Unknown"));
                                nvidia_gpu.cuda_version = nvml.sys_cuda_driver_version().unwrap_or(0).to_string();
                                nvidia_gpu.temperature = device.temperature(TemperatureSensor::Gpu).unwrap_or(0);
                                nvidia_gpu.fan_speed = device.fan_speed_rpm(0).unwrap_or(0);
                                nvidia_gpu.total_memory = device.memory_info().unwrap().total;
                                nvidia_gpu.used_memory = device.memory_info().unwrap().used;
                                nvidia_gpu.memory_utilization = device.utilization_rates().unwrap().memory;

                                nvidia_gpu_info.push(Box::new(nvidia_gpu) as Box<dyn Component>);
                            }
                        }
                    }
                    _ => { return Err(WMIError::ResultEmpty) }
                }
            }
            _ => { return Err(WMIError::ResultEmpty) }
        }

        Ok(nvidia_gpu_info)
    }
}
