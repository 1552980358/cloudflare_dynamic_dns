use crate::configuration::cloudflare::domain_name::DomainType;
use crate::rest_api::cloudflare::record::RecordType;

impl Into<RecordType> for DomainType {
    fn into(self) -> RecordType {
        match self {
            DomainType::A => RecordType::A,
            DomainType::AAAA => RecordType::AAAA
        }
    }
}

impl PartialEq<DomainType> for RecordType {
    fn eq(&self, other: &DomainType) -> bool {
        matches!((self, other), (RecordType::A, DomainType::A) | (RecordType::AAAA, DomainType::AAAA))
    }
}