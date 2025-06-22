use sysinfo::System;
use clap::Parser;
use colored::Colorize;
use crate::cli::args::Cli;
use crate::system_info::grabber::{ComponentType, grab};
use crate::utils::wmic_check::install_wmic_if_not_exist;

mod cli;
mod system;
mod system_info;
mod utils;

fn main() {
    match install_wmic_if_not_exist() {
        Ok(_) => {},
        Err(e) => { eprintln!("{}: {}","ERROR".red(), e); }
    }

    let cli = Cli::parse();
    let mut sys = System::new_all();
    let query_component_types = vec!(
        ComponentType::Cpu,
        ComponentType::Gpu,
        ComponentType::Disk,
        ComponentType::Memory,
        ComponentType::OS,
    );

    let mut info_list = Vec::new();
    for query_component_type in &query_component_types {
        let sub_info_list_result = grab(&mut sys, query_component_type);
        match sub_info_list_result {
            Ok(sub_info_list) => { info_list.extend(sub_info_list) },
            Err(_wmi_error) => { eprintln!("{}: {}","ERROR".red(), _wmi_error); }
        }
    }

    // Show information
    println!("{} \n", "Information:".green().bold());
    for info in info_list {
        println!("{}", info.get_info(cli.style));
    }
}