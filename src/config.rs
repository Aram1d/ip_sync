use log::{error, info};
use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;
use validator::{Validate, ValidationErrorsKind};

#[derive(Debug, Validate, Deserialize)]
pub struct GeneralConfig {
    #[validate(length(min = 1, message = "general.domain is required"))]
    pub domain: String,
    #[validate(range(min = 1, message = "general.poll_interval must be greater than 0"))]
    pub poll_interval: u64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct AwsConfig {
    //pub access_key: String,
    //pub secret_key: String,
    #[validate(length(min = 1, message = "aws.hosted_zone_id is required"))]
    pub hosted_zone_id: String,
    #[validate(length(min = 1, message = "aws.record_name is required"))]
    pub record_name: String,
    #[validate(range(min = 1, message = "aws.ttl must be greater than 0"))]
    pub record_ttl: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Config {
    #[validate(nested)]
    pub general: GeneralConfig,
    #[validate(nested)]
    pub aws: AwsConfig,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn load_config() -> Config {
    let config_content: String = fs::read_to_string("./config.toml")
        .map_err(|_| error!("Unable to read general config file"))
        .unwrap();
    let cfg = toml::de::from_str::<Config>(&config_content)
        .map_err(|e| error!("Unable to parse general config file: {} ", e.message()))
        .unwrap();

    if let Err(errors) = cfg.validate() {
        for (_, nested_errors) in errors.errors() {
            match nested_errors {
                ValidationErrorsKind::Struct(e) => {
                    for (_, field_errors) in e.field_errors() {
                        field_errors.iter().for_each(|e| match &e.message {
                            Some(msg) => error!("Incorrect configuration: {}", msg),
                            None => {}
                        });
                    }
                }
                _ => {}
            }
        }
        std::process::exit(1);
    }

    info!("========================================");
    info!("     🖧    Welcome to IPSync!    🌐     ");
    info!("========================================");
    info!(
        "Loaded config: sync {} Ipv4 for hosted zone {}, {}s polling.",
        cfg.general.domain, cfg.aws.hosted_zone_id, cfg.general.poll_interval
    );
    return cfg;
}

pub fn get_config() -> &'static Config {
    &CONFIG.get_or_init(load_config)
}
