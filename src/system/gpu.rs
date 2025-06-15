use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

#[derive(Debug)]
pub struct GpuInfo {
    name: String,                   // GPU name
    driver: String,                 // Driver version
    adapter_ram: u64,               // GPU memory size, Byte
    status: String,                 // Device status
    horizontal_resolution: u16,     // Current horizontal resolution
    vertical_resolution: u16,       // Current vertical resolution
}

impl GpuInfo {
    pub fn new(
        name: String,
        driver: String,
        adapter_ram: u64,
        status: String,
        horizontal_resolution: u16,
        vertical_resolution: u16,
    ) -> GpuInfo {
        GpuInfo {
            name, driver, adapter_ram, status, horizontal_resolution, vertical_resolution
        }
    }
}

impl Component for GpuInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        let mut info = String::from("GPU:");

        // Write GPU name
        info = format!("{} {}", info, self.name);

        if style == DisplayStyle::Minimal {
            return info;
        }

        // Write GPU RAM and driver version
        info = format!("{}\n\t- VRAM: {:.1}GB", info, self.adapter_ram as f64 / 1024u64.pow(3) as f64);
        info = format!("{}\n\t- Driver version: {}", info, self.driver);

        if style == DisplayStyle::Default {
            return info;
        }

        info = format!("{}\n\t- Status: {}", info, self.status);
        info = format!("{}\n\t- Resolution: {}x{}", info, self.horizontal_resolution, self.vertical_resolution);

        info
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use crate::cli::args::DisplayStyle;
    use crate::system::component::Component;
    use crate::system::gpu::GpuInfo;

    #[test]
    fn it_works() {
        let gpu = GpuInfo::new(
            String::from("Nvidia RTX4060 GPU"),
            String::from("30.0.15.1259"),
            8589934592u64,
            String::from("OK"),
            2560,
            1980,
        );

        assert_eq!(gpu.name, "Nvidia RTX4060 GPU");
        assert_eq!(gpu.driver, "30.0.15.1259");
        assert_eq!(gpu.adapter_ram, 8589934592u64);
        assert_eq!(gpu.status, "OK");
        assert_eq!(gpu.horizontal_resolution, 2560);
        assert_eq!(gpu.vertical_resolution, 1980);

        println!("{}", gpu.get_info(DisplayStyle::Detailed));
    }

    #[test]
    fn get_gpu_info() {
        let output = Command::new("wmic")
            .args([
                "path",
                "win32_VideoController",
                "get",
                "Name,AdapterRAM,DriverVersion,CurrentHorizontalResolution,CurrentVerticalResolution,Status",
                "/format:csv",
            ])
            .output()
            .ok()
            .unwrap();

        let output = String::from_utf8(output.stdout).ok();
        let output = output.unwrap();

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

            for (index, property_name) in header.iter().enumerate() {
                match property_name.as_str().trim() {
                    "Name" => { name = String::from(&data_table[index]); },
                    "DriverVersion" => { driver = String::from(&data_table[index]); },
                    "AdapterRAM" => { adapter_ram = u64::from_str_radix(&data_table[index], 10).unwrap() },
                    "Status" => { status = String::from(&data_table[index]); },
                    "CurrentHorizontalResolution" => { horizontal_resolution = u16::from_str_radix(&data_table[index], 10).unwrap() },
                    "CurrentVerticalResolution" => { vertical_resolution = u16::from_str_radix(&data_table[index], 10).unwrap() },

                    _ => {},
                }
            }

            // pack properties into struct
            let info = GpuInfo::new(
                name, driver, adapter_ram, status, horizontal_resolution, vertical_resolution
            );
            println!("{}", info.get_info(DisplayStyle::Detailed));
        }
    }
}