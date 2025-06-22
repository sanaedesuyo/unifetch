use colored::Colorize;
use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

#[derive(Debug)]
pub struct NvidiaGpuInfo {
    pub name: String,
    pub driver_version: String,
    pub cuda_version: String,
    pub temperature: u32,
    pub fan_speed: u32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub memory_utilization: u32,
}

impl NvidiaGpuInfo {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        driver_version: String,
        cuda_version: String,
        temperature: u32,
        fan_speed: u32,
        total_memory: u64,
        used_memory: u64,
        memory_utilization: u32,
    ) -> NvidiaGpuInfo {
        NvidiaGpuInfo {
            name,
            driver_version,
            cuda_version,
            temperature,
            fan_speed,
            total_memory,
            used_memory,
            memory_utilization,
        }
    }

    pub fn default() -> NvidiaGpuInfo {
        NvidiaGpuInfo {
            name: String::new(),
            driver_version: String::new(),
            cuda_version: String::new(),
            temperature: 0,
            fan_speed: 0,
            total_memory: 0,
            used_memory: 0,
            memory_utilization: 0,
        }
    }
}

impl Component for NvidiaGpuInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        let info = format!("{}:", "Nvidia GPU".blue());

        // Write Nvidia GPU name
        let info = format!("{} {}", info, self.name);

        if style == DisplayStyle::Minimal {
            return info;
        }

        // Write driver version, cuda version and memory utilization.
        let info = format!("{}\n\t- {}: {}", info, "Driver version".yellow(), self.driver_version);
        let info = format!("{}\n\t- {}: {}", info, "CUDA version".yellow(), self.cuda_version);
        let info = format!("{}\n\t- {}: {:.1}%", info, "Memory utilization".yellow(), self.memory_utilization);

        if style == DisplayStyle::Default {
            return info;
        }

        // Write temperature, fan speed and memory information
        let info = format!("{}\n\t- {}: {:.1}GB", info, "Total memory".yellow(),
            self.total_memory as f64 / 1024f64.powi(3)
        );
        let info = format!("{}\n\t- {}: {:.1}GB", info, "Used memory".yellow(),
                           self.used_memory as f64 / 1024f64.powi(3)
        );
        let info = format!("{}\n\t- {}: {}°C", info, "Temperature".yellow(), self.temperature);
        if self.fan_speed.eq(&0) {
            return info;
        }

        let info = format!("{}\n\t- {}: {}rpm", info, "Fan speed".yellow(), self.fan_speed);

        info
    }
}