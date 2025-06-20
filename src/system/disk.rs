use colored::Colorize;
use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

#[derive(Debug)]
pub struct DiskInfo {
    name: String,
    file_system: String,
    total_space: u64,       // Byte
    available_space: u64,   // Byte
    disk_type: String,
}

impl DiskInfo {
    pub fn new(name: String, file_system: String, total_space: u64, available_space: u64, disk_type: String) -> DiskInfo {
        DiskInfo {
            name,
            file_system,
            total_space,
            available_space,
            disk_type,
        }
    }
}

impl Component for DiskInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        let info = format!("{}:", "Disk".blue());

        // Write disk name
        let info = format!("{} {}", info, self.name);

        if style == DisplayStyle::Minimal {
            return info;
        }

        // Write disk total space and available space
        let info = format!("{}\n\t- {}: {:.2}GB", info, "Total space".yellow(), self.total_space as f64 / 1024f64.powi(3));
        let info = format!("{}\n\t- {}: {:.2}GB", info, "Available space".yellow(), self.available_space as f64 / 1024f64.powi(3));
        let info = format!("{}\n\t- {}: {:.2}%", info, "Occupancy".yellow(), 100f64 - self.available_space as f64 / self.total_space as f64 * 100f64);

        if style == DisplayStyle::Default {
            return info;
        }

        // Write disk file system and disk type
        let info = format!("{}\n\t- {}: {}", info, "Disk type".yellow(), self.disk_type);
        let info = format!("{}\n\t- {}: {}", info, "File system".yellow(), self.file_system);

        info
    }
}

#[cfg(test)]
mod tests {
    use sysinfo::{Disks, System};
    use crate::system::disk::DiskInfo;

    #[test]
    fn it_works() {
        let disk_info = DiskInfo::new(
            "SAMSUNG".to_string(),
            "NTFS".to_string(),
            8196u64.pow(3),
            2048u64.pow(3),
            "SSD".to_string(),
        );

        assert_eq!(disk_info.name, "SAMSUNG");
        assert_eq!(disk_info.total_space, 8196u64.pow(3));
        assert_eq!(disk_info.available_space, 2048u64.pow(3));
        assert_eq!(disk_info.file_system, "NTFS");

        println!("{:#?}", Disks::new_with_refreshed_list().list());

        let sys = System::new_all();
    }
}
