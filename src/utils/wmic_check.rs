use std::process::Command;
use wmi::WMIError;

fn is_wmic_installed() -> bool {
    let output = Command::new("cmd")
        .args(&["/C", "wmic os get caption /value"])
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn install_wmic() -> Result<(), WMIError> {
    let output = Command::new("DISM")
        .args(&[
            "/Online",
            "/Add-Capability",
            "/CapabilityName:WMIC~~~~"
        ])
        .output();

    match output {
        Ok(_) => { Ok(()) },
        Err(message) => Err(WMIError::SerdeError(format!(
            "{}\n{}",
            "failed to install wmic with DISM.",
            message
        )))
    }
}

pub fn install_wmic_if_not_exist() -> Result<(), WMIError> {
    match is_wmic_installed() {
        true => Ok(()),
        false => {
            println!("WMIC not exist. Installing WMIC...");
            install_wmic()
        }
    }
}