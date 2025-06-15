use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

#[derive(Debug)]
pub struct CpuInfo {
    name: String,
    cores: usize,
    usage: f32,
    frequency: u64,
}

impl CpuInfo {
    pub fn new(
        name: String,
        cores: usize,
        usage: f32,
        frequency: u64,
    ) -> CpuInfo {
        CpuInfo {
            name,
            cores,
            usage,
            frequency,
        }
    }
}

impl Component for CpuInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        let mut info = String::from("CPU:");

        // Write CPU name
        info = format!("{} {}", info, self.name);

        if style == DisplayStyle::Minimal {
            return info;
        }

        // Write CPU cores
        info = format!("{}\n\t- Cores: {}", info, self.cores);

        if style == DisplayStyle::Default {
            return info;
        }

        // Write CPU usage
        info = format!("{}\n\t- Usage: {:.1}%", info, self.usage);
        // Write CPU frequency
        info = format!("{}\n\t- Frequency: {}MHz", info, self.frequency);

        info
    }
}

#[cfg(test)]
mod tests {
    use sysinfo::System;
    use crate::cli::args::DisplayStyle;
    use crate::system::component::Component;
    use crate::system::cpu::CpuInfo;

    #[test]
    fn it_works() {
        let cpu = CpuInfo::new(
            "Intel".into(),
            8,
            21.9,
            120000,
        );

        assert_eq!(cpu.name, "Intel".to_string());
        assert_eq!(cpu.cores, 8);
        assert_eq!(cpu.usage, 21.9);
        assert_eq!(cpu.frequency, 120000);

        println!("{}", cpu.get_info(DisplayStyle::Detailed));
    }

    #[test]
    fn get_cpu_info() {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu = sys.cpus().first().unwrap();

        println!("{:#?}", cpu);
    }
}