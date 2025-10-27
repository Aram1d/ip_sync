use crate::config::DnsConfig;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType,
};
use aws_sdk_route53::Client;
use colored::*;
use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;
use tokio::sync::OnceCell;

static CLIENTS: OnceCell<Mutex<HashMap<String, Client>>> = OnceCell::const_new();

async fn get_clients() -> &'static Mutex<HashMap<String, Client>> {
    CLIENTS
        .get_or_init(|| async { Mutex::new(HashMap::new()) })
        .await
}

async fn get_client(dns_config: &DnsConfig) -> Client {
    let mut clients = get_clients().await.lock().unwrap();
    
    if let Some(client) = clients.get(&dns_config.domain) {
        return client.clone();
    }

    let aws_client_config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(Credentials::from_keys(
            &dns_config.aws.access_key,
            &dns_config.aws.secret_key,
            None,
        ))
        .load()
        .await;
    let client = Client::new(&aws_client_config);
    clients.insert(dns_config.domain.clone(), client.clone());
    client
}

pub async fn get_ip(dns_config: &DnsConfig) -> Result<String, Box<dyn Error>> {
    match get_client(dns_config)
        .await
        .list_resource_record_sets()
        .hosted_zone_id(&dns_config.aws.hosted_zone_id)
        .send()
        .await
    {
        Ok(output) => {
            let record = output
                .resource_record_sets
                .iter()
                .find(|r| r.name == dns_config.aws.record_name.to_string() + ".");
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

pub async fn update_record(dns_config: &DnsConfig, ipv4: &str) -> Result<(), Box<dyn Error>> {
    // Create the resource record set
    let record = ResourceRecordSet::builder()
        .name(&dns_config.aws.record_name)
        .r#type(RrType::A)
        .ttl(dns_config.aws.record_ttl)
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
    return match get_client(dns_config)
        .await
        .change_resource_record_sets()
        .hosted_zone_id(&dns_config.aws.hosted_zone_id)
        .change_batch(change_batch)
        .send()
        .await
    {
        Ok(_) => {
            info!(
                "Dns record {} on HZ {} updated to {}",
                dns_config.aws.record_name.bold(),
                dns_config.aws.hosted_zone_id.bold(),
                ipv4.bold().green()
            );
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    };
}
