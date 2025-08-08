use std::{fmt::Display, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::VariantArray;

use crate::CmmError;

#[derive(
    VariantArray,
    Hash,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    PartialOrd,
    Ord,
)]
pub enum Domain {
    Business,
    People,
    Process,
    Technology,
    Services,
}

#[derive(
    Debug, SerializeDisplay, DeserializeFromStr, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord,
)]
pub struct CID {
    domain: Domain,
    id: [u8; 4],
}

impl Display for CID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.domain, self.as_short_string())
    }
}

impl CID {
    pub fn aspect_id(&self) -> u8 {
        self.id[0]
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }

    pub fn indent(&self) -> usize {
        self.id.iter().filter(|&id| *id != 0u8).count()
    }

    pub fn as_short_string(&self) -> String {
        self.id.iter().filter(|&id| *id != 0u8).join(".")
    }
}

impl FromStr for CID {
    type Err = CmmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(".");
        let domain = parts.next().ok_or(CmmError::CIDMissingDomain)?.parse()?;
        let mut ids: [u8; 4] = [0; 4];
        for id in ids.iter_mut() {
            if let Some(part) = parts.next() {
                *id = part.parse::<u8>()?;
                if *id == 0 {
                    return Err(CmmError::CIDInvalidZero);
                }
            } else {
                *id = 0;
            };
        }
        Ok(CID { domain, id: ids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_from_string() {
        let cid = "Business.1.2.3.4".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 2, 3, 4]);
    }

    #[test]
    fn test_short_cid() {
        let cid = "Business.1".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 0, 0, 0]);
    }

    #[test]
    fn test_long_cid() {
        let cid = "Business.11.11.11.11".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [11, 11, 11, 11]);
    }

    #[test]
    fn test_cid_invalid_domain() {
        assert!("A.1.2.3.4".parse::<CID>().is_err());
    }

    #[test]
    fn test_cid_invalid_zero() {
        assert!("Business.0.11.11.11".parse::<CID>().is_err());
    }

    #[test]
    fn test_cid_serialization() {
        assert_eq!(
            "Business.1",
            "Business.1".parse::<CID>().unwrap().to_string()
        );
    }

    #[test]
    fn test_cid_sort() {
        let mut cids = vec![
            "Services.1.1".parse().unwrap(),
            "Services.1.11".parse().unwrap(),
            "Services.1.2".parse().unwrap(),
            "People.3".parse().unwrap(),
        ];

        let correct_order: Vec<CID> = vec![
            "People.3".parse().unwrap(),
            "Services.1.1".parse().unwrap(),
            "Services.1.2".parse().unwrap(),
            "Services.1.11".parse().unwrap(),
        ];

        cids.sort();

        assert_eq!(correct_order, cids);
    }

    #[test]
    fn test_cid_as_short_string() {
        assert_eq!("People.3".parse::<CID>().unwrap().as_short_string(), "3".to_string());
        assert_eq!("People.3.1".parse::<CID>().unwrap().as_short_string(), "3.1".to_string());
        assert_eq!("People.3.12.1".parse::<CID>().unwrap().as_short_string(), "3.12.1".to_string());
    }

    #[test]
    fn test_cid_indent() {
        assert_eq!("People.3".parse::<CID>().unwrap().indent(), 1);
        assert_eq!("People.3.1".parse::<CID>().unwrap().indent(), 2);
        assert_eq!("People.3.12.1".parse::<CID>().unwrap().indent(), 3);
    }
}
