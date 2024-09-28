use crate::config::get_config;
use lazy_regex::regex;
use lazy_regex::Lazy;
use std::error::Error;
use std::process::Command;

static IP_REGEX: &Lazy<lazy_regex::Regex> =
    regex!(r"ExternalIPAddress = (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\n");

pub fn get_domain_ip() -> Result<String, Box<dyn Error>> {
    let domain = &get_config().general.domain;

    let output = Command::new("dig")
        .arg("+short")
        .arg(domain)
        .output()
        .map_err(|e| format!("Failed to execute dig: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return Ok(stdout);
        } else {
            return Err("No IP address found.".into());
        }
    } else {
        return Err(Box::from(format!("{:?}", output.status)));
    }
}

pub fn get_actual_ip() -> Result<String, Box<dyn Error>> {
    let output = Command::new("upnpc")
        .arg("-s")
        .output()
        .map_err(|e| format!("Failed to execute upnpc: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        match IP_REGEX.captures(&stdout) {
            Some(captures) => {
                return Ok(captures.get(1).unwrap().as_str().to_string());
            }
            None => {
                return Err(Box::from("No IP address found."));
            }
        }
    } else {
        return Err(Box::from(format!("{:?}", output.status)));
    }
}
