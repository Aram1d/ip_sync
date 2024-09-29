mod cli_args;
mod config;
mod ip_utils;
mod logger;
mod route53;
mod utils;
use config::get_config;
use ip_utils::{get_actual_ip, get_domain_ip};
use std::error::Error;
use utils::map_prefixed_err;

use log::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init_logger();
    let config = &get_config().general;

    info!("Starting watching ip changes");

    loop {
        match async {
            let domain_ip = get_domain_ip()
                .await
                .map_err(utils::map_prefixed_err("Failed to get domain IP:"))?;

            let actual_ip =
                get_actual_ip().map_err(map_prefixed_err("Failed to get current IP:"))?;

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
            Ok::<(), Box<dyn Error>>(())
        }
        .await
        {
            Ok(_) => {}
            Err(e) => {
                error!("{e}");
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(config.poll_interval));
    }
}
