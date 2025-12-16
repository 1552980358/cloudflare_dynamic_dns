mod rest_api;
mod configuration;
mod record_types;

#[tokio::main]
async fn main() {
    use configuration::Configuration;
    let configuration = Configuration::new()
        .unwrap_or_else(|error| {
            use configuration::error::Error;
            let error_message = match error {
                Error::CloudflareImportFail(path) => { format!("Failed to import Cloudflare JSON config from {path}") }
                Error::ConfigImportFail(path) => { format!("Failed to import JSON config from {path}") }
            };

            use log::error;
            error!(target: "main", "{error_message}");
            panic!("{error_message}");
        });

    let (token, zone, (total, connect, read)) = (
        &configuration.cloudflare.token, &configuration.cloudflare.zone, configuration.config.ip_sb_timeout.all()
    );
    use rest_api::CloudflareApi;
    let cloudflare_api = CloudflareApi::new(&token, &zone, total, connect, read);
    if let Err(error) = cloudflare_api.verify_user_token().await {
        use rest_api::cloudflare::error::Error;
        let error_message = match error {
            Error::Network => { "Network error occurred when verifying token availability from Cloudflare api" }
            Error::Unauthorized => { "Unauthorized responded when verifying token availability from Cloudflare api" }
            Error::DecodeResponse => { "Unknown response body responded when verifying token availability from Cloudflare api" }
            Error::Unknown => { "Unknown error occurred when verifying token availability from Cloudflare api" }
            _ => unreachable!("Unreachable condition met when handing error from verifying token availability from Cloudflare api")
        };

        use log::error;
        error!(target: "main", "{error_message}");
        panic!("{}", error_message);
    };

    let records = cloudflare_api.list_record().await
        .unwrap_or_else(|error| {
            use rest_api::cloudflare::error::Error;
            let error_message = match error {
                Error::Internal => { "Internal error caused due to invalid request content sent to Cloudflare api" }
                Error::Network => { "Network error occurred when requesting record list from Cloudflare api" }
                Error::Unauthorized => { "Unauthorized responded when requesting record list from Cloudflare api" }
                Error::InvalidZone => { "Invalid zone responded when requesting record list from Cloudflare api" }
                Error::Server => { "Server error occurred when requesting record list from Cloudflare api" }
                Error::DecodeResponse => { "Deserializing error occurred when processing listed records response from Cloudflare api" }
                Error::Unknown => { "Unknown error occurred when requesting record list from Cloudflare api" }
                _ => unreachable!("Unreachable condition met when handing error from requesting list of records from Cloudflare api")
            };

            use log::error;
            error!(target: "main", "{error_message}");
            panic!("{error_message}");
        });

    if let Some(is_proxied) = configuration.proxied {
        let domain_names = &configuration.cloudflare.domain_names;
        handle_proxied(&cloudflare_api, &records, &domain_names, is_proxied).await;
    }
    else {
        use rest_api::IpSBApi;
        let ip = IpSBApi::new().get_ip().await
            .unwrap_or_else(|error| {
                use rest_api::ip_sb::error::Error;
                let error_message = match error {
                    Error::Network => { "Network error occurred when sending request to ip.sb api" }
                    Error::Server => { "Server error responded when requesting ip address from ip.sb api" }
                    Error::DecodeResponse => { "Deserializing error occurred when processing ip.sb api json response" }
                    Error::Unknown => { "Unknown error occurred when requesting ip address from ip.sb api" }
                };

                use log::error;
                error!(target: "main", "{error_message}");
                panic!("{error_message}");
            });

        let (domain_names, unavailable_hide) = (
            &configuration.cloudflare.domain_names, configuration.config.unavailable_hide
        );
        handle_ip_update(&cloudflare_api, &records, &domain_names, &ip, unavailable_hide).await;
    }
}

use rest_api::{cloudflare::record::Record, CloudflareApi};
use configuration::cloudflare::domain_name::DomainName;

#[inline]
async fn handle_proxied(cloudflare_api: &CloudflareApi, records: &Vec<Record>, domain_names: &Vec<DomainName>, is_proxied: bool) {
    for domain_name in domain_names {
        let record = records.iter()
            .find(|record| record.domain_name == domain_name.name && record.record_type == domain_name.domain_type);
        if let Some(record) = record && record.proxied != is_proxied {
            handle_record_proxied_update(&cloudflare_api, &record.id, is_proxied).await;
        }
    }
}

#[inline]
async fn handle_record_proxied_update(cloudflare_api: &CloudflareApi, record_id: &String, is_proxied: bool) {
    match cloudflare_api.update_record_proxied(&record_id, is_proxied).await {
        Ok(record) => {
            if record.id == *record_id && record.proxied != is_proxied {
                use log::info;
                info!(target: "main", "Update {} ({}) proxied -> {}", record.domain_name, record.record_type, is_proxied);
            }
            else {
                use log::error;
                error!(target: "main", "Failed to update {} ({}) record proxied", record.domain_name, record.record_type);
            }
        }
        Err(error) => {
            use rest_api::cloudflare::error::Error;
            let error_message = match error {
                Error::Internal => { "Internal error caused due to invalid proxied update request sent to Cloudflare api" }
                Error::Network => { "Network error occurred when updating record proxied value from Cloudflare api" }
                Error::Unauthorized => { "Unauthorized responded when updating record proxied value from Cloudflare api" }
                Error::InvalidZone => { "Invalid zone responded when updating record proxied value from Cloudflare api" }
                Error::InvalidRecord => { "Invalid record id responded when updating record proxied value from Cloudflare api" }
                Error::Server => { "Server error occurred when updating record proxied value from Cloudflare api" }
                Error::DecodeResponse => { "Deserializing error occurred when processing record proxied update response from Cloudflare api" }
                Error::Unknown => { "Unknown error occurred when updating record proxied value from Cloudflare api" }
            };

            use log::error;
            error!(target: "main", "{error_message}");
        }
    }
}

use rest_api::ip_sb::ip::IP;

#[inline]
async fn handle_ip_update(cloudflare_api: &CloudflareApi, records: &Vec<Record>, domain_names: &Vec<DomainName>, ip: &IP, unavailable_hide: bool) {
    for domain_name in domain_names {
        let record = records.iter()
            .find(|record| record.domain_name == domain_name.name && record.record_type == domain_name.domain_type);

        if let Some(record) = record {
            use configuration::cloudflare::domain_name::DomainType;
            if let Some(ip) = match domain_name.domain_type { DomainType::A => ip.v4(), DomainType::AAAA => ip.v6() } {
                // Do update ip
                handle_record_ip_update(&cloudflare_api, &record.id, &ip).await;
            }
            else if unavailable_hide {
                handle_record_delete(&cloudflare_api, &record.id).await;
            }
        }
        else {
            use configuration::cloudflare::domain_name::DomainType;
            if let Some(ip) = match domain_name.domain_type { DomainType::A => ip.v4(), DomainType::AAAA => ip.v6() } {
                // Do record creation
                handle_record_create(&cloudflare_api, &domain_name, &ip).await;
            }
        }
    }
}

#[inline]
async fn handle_record_ip_update(cloudflare_api: &CloudflareApi, record_id: &String, ip: &String) {
    match cloudflare_api.update_record_value(&record_id, &ip).await {
        Ok(record) => {
            if record.id == *record_id && record.value == *ip {
                use log::info;
                info!(target: "main", "Update {} ({}) record ip -> {}", record.domain_name, record.record_type, ip);
            }
            else {
                use log::error;
                error!(target: "main", "Failed to update {} ({}) record ip", record.domain_name, record.record_type);
            }
        }
        Err(error) => {
            use rest_api::cloudflare::error::Error;
            let error_message = match error {
                Error::Internal => { "Internal error caused due to invalid ip update request sent to Cloudflare api" }
                Error::Network => { "Network error occurred when updating record ip value from Cloudflare api" }
                Error::Unauthorized => { "Unauthorized responded when updating record ip value from Cloudflare api" }
                Error::InvalidZone => { "Invalid zone responded when updating record ip value from Cloudflare api" }
                Error::InvalidRecord => { "Invalid record id responded when updating record ip value from Cloudflare api" }
                Error::Server => { "Server error occurred when updating record ip value from Cloudflare api" }
                Error::DecodeResponse => { "Deserializing error occurred when processing record ip update response from Cloudflare api" }
                Error::Unknown => { "Unknown error occurred when updating record ip value from Cloudflare api" }
            };

            use log::error;
            error!(target: "main", "{error_message}");
        }
    };
}

#[inline]
async fn handle_record_delete(cloudflare_api: &CloudflareApi, record_id: &String) {
    if let Err(error) = cloudflare_api.delete_record(&record_id).await {
        use rest_api::cloudflare::error::Error;
        let error_message = match error {
            Error::Internal => { "Internal error caused due to invalid delete request sent to Cloudflare api" }
            Error::Network => { "Network error occurred when deleting record value from Cloudflare api" }
            Error::Unauthorized => { "Unauthorized responded when deleting record value from Cloudflare api" }
            Error::InvalidZone => { "Invalid zone responded when deleting record value from Cloudflare api" }
            Error::InvalidRecord => { "Invalid record id responded when deleting record value from Cloudflare api" }
            Error::Server => { "Server error occurred when deleting record value from Cloudflare api" }
            Error::DecodeResponse => { "Deserializing error occurred when processing record deletion response from Cloudflare api" }
            Error::Unknown => { "Unknown error occurred when deleting record value from Cloudflare api" }
        };

        use log::error;
        error!(target: "main", "{error_message}");
    }
}

#[inline]
async fn handle_record_create(cloudflare_api: &CloudflareApi, domain_name: &DomainName, ip: &String) {
    // let domain = &domain_name.name;
    // use rest_api::cloudflare::record::RecordType;
    // let record_type: RecordType = domain_name.domain_type.into();
    // let time_to_live = domain_name.time_to_live;
    // let proxied = domain_name.proxied;
    let (domain_name, record_type, time_to_live, proxied) = (
        &domain_name.name, domain_name.domain_type.into(), domain_name.time_to_live, domain_name.proxied
    );

    match cloudflare_api.create_record(domain_name, &ip, &record_type, time_to_live, proxied).await {
        Ok(record) => {
            if record.domain_name == *domain_name && record.record_type == record_type && record.value == *ip {
                use log::info;
                info!(target: "main", "Update {} ({}) record -> {}", record.domain_name, record.record_type, ip);
            }
            else {
                use log::error;
                error!(target: "main", "Failed to update {} ({})", record.domain_name, record.record_type);
            }
        }
        Err(error) => {
            use rest_api::cloudflare::error::Error;
            let error_message = match error {
                Error::Internal => { "Internal error caused due to invalid creation request sent to Cloudflare api" }
                Error::Network => { "Network error occurred when creating record value from Cloudflare api" }
                Error::Unauthorized => { "Unauthorized responded when creating record value from Cloudflare api" }
                Error::InvalidZone => { "Invalid zone responded when creating record value from Cloudflare api" }
                Error::InvalidRecord => { "Invalid record id responded when creating record value from Cloudflare api" }
                Error::Server => { "Server error occurred when creating record value from Cloudflare api" }
                Error::DecodeResponse => { "Deserializing error occurred when processing record creation response from Cloudflare api" }
                Error::Unknown => { "Unknown error occurred when creating record value from Cloudflare api" }
            };

            use log::error;
            error!(target: "main", "{error_message}");
        }
    };
}