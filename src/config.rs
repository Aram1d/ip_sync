use crate::cli_args;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;
use validator::{Validate, ValidationErrorsKind};

#[derive(Debug, Default, Deserialize, Validate)]
pub struct GeneralConfig {
    #[validate(range(min = 1, message = "general.poll_interval must be greater than 0"))]
    pub poll_interval: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AwsConfig {
    #[validate(length(min = 1, message = "access_key is required"))]
    pub access_key: String,
    #[validate(length(min = 1, message = "secret_key is required"))]
    pub secret_key: String,
    #[validate(length(min = 1, message = "hosted_zone_id is required"))]
    pub hosted_zone_id: String,
    #[validate(length(min = 1, message = "record_name is required"))]
    pub record_name: String,
    #[validate(range(min = 1, message = "ttl must be greater than 0"))]
    pub record_ttl: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct DnsConfig {
    #[validate(length(min = 1, message = "domain is required"))]
    pub domain: String,
    #[validate(nested)]
    pub aws: AwsConfig,
}

#[derive(Debug, Default, Deserialize, Validate)]
pub struct Config {
    #[validate(nested)]
    pub general: GeneralConfig,
    #[validate(nested)]
    #[validate(length(min = 1, message = "At least one DNS configuration is required"))]
    pub dns: Vec<DnsConfig>,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

static WELCOME_MSG: &'static str = r#"
========================================
     🌐    Welcome to IpSync!    🖧    
========================================
"#;

pub static DEFAULT_CONFIG: &'static str = r#"
[general]
poll_interval = 60

[[dns]]
domain = "test.example.com"

[dns.aws]
access_key = "your_access_key"
secret_key = "your_secret_key"
hosted_zone_id = "your_zone_id"
record_name = "your_a_record_name"
record_ttl = 60

[[dns]]
domain = "another.example.com"

[dns.aws]
access_key = "your_access_key"
secret_key = "your_secret_key"
hosted_zone_id = "your_zone_id"
record_name = "your_a_record_name"
record_ttl = 60
"#;

pub static MISSING_CONFIG: &'static str = r#"
Error: Configuration file not found.

To proceed, you have the following options:
1. Create a configuration file at the default location: `/etc/ipsync.conf`*.
2. Specify a custom configuration path using the `-c` flag:
   Example: `ipsync -c /your/custom/path.conf`.

*You can generate a default configuration:
Run `ipsync -g > /etc/ipsync.conf` to create one from a model.
**Important**: After generating the file, open it and edit the settings to match your environment (e.g., domain, AWS credentials, etc.).
"#;

fn load_config() -> Config {
    let config_path = cli_args::get_args().get_one::<String>("config");

    let config_content: String = fs::read_to_string(
        config_path.unwrap_or(&"/etc/ipsync.conf".to_string()),
    )
    .unwrap_or_else(|_| {
        error!("{MISSING_CONFIG}");
        std::process::exit(1)
    });

    let cfg = toml::de::from_str::<Config>(&config_content).unwrap_or_else(|e| {
        error!("Unable to parse general config file: {} ", e.message());
        std::process::exit(1)
    });

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

    info!("{WELCOME_MSG}");
    info!(
        "Config loaded: {} DNS configuration(s), {}s polling",
        cfg.dns.len(),
        cfg.general.poll_interval
    );
    for dns_cfg in &cfg.dns {
        info!(
            "  - Domain: {}, hosted zone: {}",
            dns_cfg.domain, dns_cfg.aws.hosted_zone_id
        );
    }
    return cfg;
}

pub fn get_config() -> &'static Config {
    &CONFIG.get_or_init(load_config)
}
