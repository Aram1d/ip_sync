use crate::config::get_config;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType,
};
use aws_sdk_route53::Client;
use colored::*;
use log::info;
use std::error::Error;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn load_client() -> Client {
    let config = &get_config().aws;
    let aws_client_config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(Credentials::from_keys(
            &config.access_key,
            &config.secret_key,
            None,
        ))
        .load()
        .await;
    Client::new(&aws_client_config)
}

async fn get_client() -> &'static Client {
    CLIENT.get_or_init(load_client).await
}

pub async fn get_ip() -> Result<String, Box<dyn Error>> {
    let config = &get_config().aws;

    match get_client()
        .await
        .list_resource_record_sets()
        .hosted_zone_id(&config.hosted_zone_id)
        .send()
        .await
    {
        Ok(output) => {
            let record = output
                .resource_record_sets
                .iter()
                .find(|r| r.name == config.record_name.to_string() + ".");
            let ip: String = record
                .ok_or("ResourceRecordSet not found")?
                .resource_records
                .as_ref()
                .ok_or("ResourceRecord vector not found")?
                .first()
                .ok_or("ResourceRecord not found")?
                .value
                .to_string();
            return Ok(ip);
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn update_record(ipv4: &str) -> Result<(), Box<dyn Error>> {
    let config = &get_config().aws;

    // Create the resource record set
    let record = ResourceRecordSet::builder()
        .name(&config.record_name)
        .r#type(RrType::A)
        .ttl(config.record_ttl)
        .resource_records(ResourceRecord::builder().value(ipv4).build()?)
        .build()?;

    // Create the change batch
    let change_batch = ChangeBatch::builder()
        .changes(
            Change::builder()
                .action(ChangeAction::Upsert)
                .resource_record_set(record)
                .build()?,
        )
        .build()?;

    // Send the request
    return match get_client()
        .await
        .change_resource_record_sets()
        .hosted_zone_id(&config.hosted_zone_id)
        .change_batch(change_batch)
        .send()
        .await
    {
        Ok(_) => {
            info!(
                "Dns record {} on HZ {} updated to {}",
                config.record_name.bold(),
                config.hosted_zone_id.bold(),
                ipv4.bold().green()
            );
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    };
}
