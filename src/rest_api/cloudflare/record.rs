mod list;
mod create;
mod delete;
mod update;

mod handle_network_error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Record {
    pub id: String,
    #[serde(rename = "name")]
    pub domain_name: String,
    #[serde(rename = "content")]
    pub value: String,
    #[serde(rename = "type")]
    pub record_type: RecordType,
    pub proxied: bool
}

#[derive(Serialize, Deserialize)]
pub enum RecordType { A, AAAA, NS }