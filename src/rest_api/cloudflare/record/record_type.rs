use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum RecordType { A, AAAA, NS }

impl PartialEq for RecordType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (RecordType::A, RecordType::A) | (RecordType::AAAA, RecordType::AAAA) | (RecordType::NS, RecordType::NS)
        )
    }
}

use std::ops::Deref;

impl Deref for RecordType {

    type Target = Self;

    fn deref(&self) -> &Self::Target {
        // Just return self, as all RecordType are singleton
        &self
    }

}

const RECORD_TYPE_A: &str = "A";
const RECORD_TYPE_AAAA: &str = "A";
const RECORD_TYPE_NS: &str = "NS";

use std::fmt::{Display, Formatter};

impl Display for RecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RecordType::A => { RECORD_TYPE_A }
            RecordType::AAAA => { RECORD_TYPE_AAAA }
            RecordType::NS => { RECORD_TYPE_NS }
        };
        f.write_str(str)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_record_type_display() {
        use super::RecordType;
        assert_eq!(format!("{}", RecordType::A), super::RECORD_TYPE_A);
        assert_eq!(format!("{}", RecordType::AAAA), super::RECORD_TYPE_AAAA);
        assert_eq!(format!("{}", RecordType::NS), super::RECORD_TYPE_NS);
    }
}