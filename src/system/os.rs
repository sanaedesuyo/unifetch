use colored::Colorize;
use crate::cli::args::DisplayStyle;
use crate::system::component::Component;

pub struct OSInfo {
    name: String,
    host_name: String,

}

impl OSInfo {
    pub fn new(
        name: String,
        host_name: String,
    ) -> OSInfo {
        OSInfo {
            name,
            host_name,
        }
    }
}

impl Component for OSInfo {
    fn get_info(&self, style: DisplayStyle) -> String {
        // Write OS name
        let info = format!("{}: {}", "OS".blue(), self.name);

        if style == DisplayStyle::Minimal {
            return info;
        }

        // Write host name
        let info = format!("{}\n\t- {}: {}", info, "Host name".yellow(), self.host_name);

        info
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::args::DisplayStyle;
    use crate::system::component::Component;
    use crate::system::os::OSInfo;

    #[test]
    fn sys_info_test() {
        let info = OSInfo::new(
            "Windows 11 Home China".into(),
            "Kochiya早苗".into(),
        );

        println!("{}", info.get_info(DisplayStyle::Minimal));
    }
}