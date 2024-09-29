use crate::config::get_config;
use hickory_resolver::config::*;
use hickory_resolver::TokioAsyncResolver;
use igd_next::search_gateway;
use igd_next::SearchOptions;
use std::error::Error;

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
    let gateway = search_gateway(SearchOptions::default())?;
    let ip = gateway.get_external_ip();

    match ip {
        Ok(ip) => match ip.is_ipv4() {
            true => Ok(ip.to_string()),
            false => Err(Box::from("Ipv6 is not yet supported.")),
        },
        Err(e) => Err(Box::from(format!("{:?}", e))),
    }
}
