use hickory_resolver::proto::rr::RData;
use hickory_resolver::Resolver;
use igd_next::search_gateway;
use igd_next::SearchOptions;
use std::error::Error;
use std::net::Ipv4Addr;

pub async fn get_domain_ip(domain: &str) -> Result<String, Box<dyn Error>> {
    let resolver = Resolver::builder_tokio()?.build()?;

    let lookup = resolver.ipv4_lookup(domain).await?;
    let ipv4 = lookup.answers().iter().find_map(|r| match &r.data {
        RData::A(a) => Some(Ipv4Addr::from(*a)),
        _ => None,
    });

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
