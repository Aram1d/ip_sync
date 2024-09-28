mod config;
mod ip_utils;
mod logger;
mod route53;
use config::get_config;
use ip_utils::{get_actual_ip, get_domain_ip};
use std::error::Error;

use log::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init_logger();
    let config = &get_config().general;

    info!("Starting watching ip changes");

    loop {
        let domain_ip = get_domain_ip().map_err(|e| {
            error!("Failed to get domain IP: {}", e);
            return e;
        })?;

        let actual_ip = get_actual_ip().map_err(|e| {
            error!("Failed to get actual IP: {}", e);
            return e;
        })?;

        if actual_ip == domain_ip {
            info!(
                "IP is sync ({}) for {}, skipping...",
                actual_ip, config.domain
            );
        } else {
            warn!(
                "Actual ({}) differs from domain ({}), checking...",
                actual_ip, domain_ip
            );

            let dns_record_ip = route53::get_ip().await?;

            if dns_record_ip == actual_ip {
                info!(
                    "IP is sync ({}) for {}, wait for propagation...",
                    actual_ip, config.domain
                );
            } else {
                warn!(
                    "DNS record for {} is stale ({}), updating to {}...",
                    config.domain, dns_record_ip, actual_ip
                );
                route53::update_record(&actual_ip).await?;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(config.poll_interval));
    }
}
