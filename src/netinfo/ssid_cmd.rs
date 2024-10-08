use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

use crate::constants::CMD_ENV;

pub fn get_ssid(wifi_interface: &str) -> std::io::Result<Option<String>> {
    if let Ok(Some(v)) = get_iw_ssid(wifi_interface) {
        return Ok(Some(v));
    }

    get_wpa_cli_ssid(wifi_interface)
}

/// RM1, RM2
fn get_iw_ssid(wifi_interface: &str) -> std::io::Result<Option<String>> {
    for line in cmd_iw_ssid(wifi_interface)?.lines() {
        if line.contains("ssid") {
            if let Some(val) = line.split_whitespace().nth(1) {
                return Ok(Some(val.to_string()));
            }
        }
    }
    Ok(None)
}

/// RM1, RM2
fn cmd_iw_ssid(wifi_interface: &str) -> std::io::Result<String> {
    let command_out = Command::new(CMD_ENV)
        .args(["iw", "dev", wifi_interface, "info"])
        .stdout(Stdio::piped())
        .output()?;
    String::from_utf8(command_out.stdout).map_err(|err| Error::new(ErrorKind::Other, err))
}

/// RMPP
fn get_wpa_cli_ssid(wifi_interface: &str) -> std::io::Result<Option<String>> {
    for line in cmd_wpa_cli_ssid(wifi_interface)?.lines() {
        if line.starts_with("ssid") {
            if let Some(val) = line.split("=").nth(1) {
                return Ok(Some(val.to_string()));
            }
        }
    }
    Ok(None)
}

/// RMPP
fn cmd_wpa_cli_ssid(wifi_interface: &str) -> std::io::Result<String> {
    let command_out = Command::new(CMD_ENV)
        .args(["wpa_cli", "status", "-i", wifi_interface])
        .stdout(Stdio::piped())
        .output()?;
    String::from_utf8(command_out.stdout).map_err(|err| Error::new(ErrorKind::Other, err))
}
