use strum::Display;

use serde::{Deserialize, Serialize};
use strum::VariantArray;

use crate::CmmError;

#[derive(
    VariantArray, Hash, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display,
)]
pub enum Domain {
    Business,
    People,
    Process,
    Technology,
    Services,
}

impl Domain {
    fn short(&self) -> char {
        match self {
            Domain::People => 'P',
            Domain::Business => 'B',
            Domain::Process => 'M',
            Domain::Technology => 'T',
            Domain::Services => 'S',
        }
    }
    fn from_short(char: char) -> Option<Domain> {
        match char {
            'P' => Some(Domain::People),
            'B' => Some(Domain::Business),
            'M' => Some(Domain::Process),
            'T' => Some(Domain::Technology),
            'S' => Some(Domain::Services),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CID {
    domain: Domain,
    id: [u8; 4],
}

impl CID {
    pub fn aspect_id(&self) -> u8 {
        self.id[1]
    }
}

impl TryFrom<String> for CID {
    type Error = CmmError;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let mut parts = value.split(".");
        let domain = parts
            .next()
            .and_then(|str| str.chars().next())
            .and_then(Domain::from_short)
            .ok_or(CmmError::CIDMissingDomain)?;
        let mut ids: [u8; 4] = [0; 4];
        for id in ids.iter_mut() {
            *id = u8::from_str_radix(parts.next().unwrap_or_default(), 10)?;
        }
        Ok(CID { domain, id: ids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_from_string() {
        let cid = CID::try_from("B.1.2.3.4".to_string()).unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 2, 3, 4]);
    }

    #[test]
    fn test_short_cid() {
        let cid = CID::try_from("B.1".to_string()).unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 0, 0, 0]);
    }

    #[test]
    fn test_long_cid() {
        let cid = CID::try_from("B.11.11.11.11".to_string()).unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [11, 11, 11, 11]);
    }

    #[test]
    #[should_panic]
    fn test_cid_invalid_domain() {
        assert!(CID::try_from("A.1.2.3.4".to_string()).is_err());
    }
}
