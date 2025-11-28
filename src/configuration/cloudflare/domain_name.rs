use serde::Deserialize;

#[derive(Deserialize)]
pub struct DomainName {
    pub name: String,
    #[serde(rename="record-type", default)]
    pub record_type: RecordType,
    #[serde(default = "default::proxied")]
    pub proxied: bool,
    #[serde(default = "default::time_to_live")]
    pub time_to_live: u16,
    #[serde(default)]
    pub comment: Option<String>
}

#[derive(Deserialize, Default)]
pub enum RecordType {
    #[default]
    A,
    AAAA
}

mod default {

    /**
     * Let default as `auto`, see https://developers.cloudflare.com/api/resources/dns/subresources/records/models/ttl/#(schema)
     * ```
     * Time To Live (TTL) of the DNS record in seconds. Setting to 1 means 'automatic'.
     * Value must be between 60 and 86400, with the minimum reduced to 30 for Enterprise zones.
     * ```
     **/
    pub(super) fn time_to_live() -> u16 { 1 }

    pub(super) fn proxied() -> bool { true }

}