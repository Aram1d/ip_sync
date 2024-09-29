use crate::config::get_config;
use hickory_resolver::config::*;
use hickory_resolver::TokioAsyncResolver;
use lazy_regex::regex;
use lazy_regex::Lazy;
use std::error::Error;
use std::process::Command;

static IP_REGEX: &Lazy<lazy_regex::Regex> =
    regex!(r"ExternalIPAddress = (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\n");

pub async fn get_domain_ip() -> Result<String, Box<dyn Error>> {
    let domain = &get_config().general.domain;
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let lookup = resolver.ipv4_lookup(domain).await?;
    let ipv4 = lookup.iter().next();

    match ipv4 {
        Some(ip) => Ok(ip.to_string()),
        None => Err(Box::from(format!(
            "Could not resolve {domain} to IP address."
        ))),
    }
}

pub fn get_actual_ip() -> Result<String, Box<dyn Error>> {
    let output = Command::new("upnpc")
        .arg("-s")
        .output()
        .map_err(|e| format!("{}, is upnpc installed?", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        match IP_REGEX.captures(&stdout) {
            Some(captures) => return Ok(captures.get(1).unwrap().as_str().to_string()),
            None => return Err(Box::from("No IP address found.")),
        }
    } else {
        return Err(Box::from(format!("{:?}", output.status)));
    }
}
