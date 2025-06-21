use colored::Colorize;
use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

pub struct MemoryInfo {
    total_memory: u64,      // Byte
    free_memory: u64,       // Byte,
}

impl MemoryInfo {
    pub fn new(
        total_memory: u64,
        free_memory: u64,
    ) -> MemoryInfo {
        MemoryInfo {
            total_memory,
            free_memory,
        }
    }
}

impl Component for MemoryInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        let info = format!("{}:", "Memory".blue());

        let (used, total) = (self.total_memory as f64 - self.free_memory as f64, self.total_memory as f64);

        // Write Memory use circumstance
        let info = format!("{} {:.1}GB/{:.1}GB",
            info,
            used / 1024f64.powi(3),
            total / 1024f64.powi(3),
        );

        if style == DisplayStyle::Default || style == DisplayStyle::Minimal {
            return info;
        }

        let info = format!("{}\n\t- {}: {:.2}%", info, "Occupancy".yellow(), used / total * 100f64);

        info
    }
}

#[cfg(test)]
mod tests {
    use sysinfo::System;

    #[test]
    fn it_works() {
        let sys = System::new_all();

        println!("{:#?}", sys.total_memory());
    }
}