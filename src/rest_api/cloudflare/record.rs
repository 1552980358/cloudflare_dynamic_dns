use serde::Deserialize;

mod list;
mod create;
mod delete;
mod handle_network_error;
mod record_type;
mod update;

pub use record_type::RecordType;

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
