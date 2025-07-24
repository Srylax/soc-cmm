use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct CID {
    domain: Domain,
    id: [u8; 4],
}

impl Display for CID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.domain, self.id[0], self.id[1], self.id[2], self.id[3]
        )
    }
}

impl CID {
    pub fn aspect_id(&self) -> u8 {
        self.id[1]
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }
}

impl FromStr for CID {
    type Err = CmmError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(".");
        let domain = parts
            .next()
            .and_then(|str| str.chars().next())
            .and_then(Domain::from_short)
            .ok_or(CmmError::CIDMissingDomain)?;
        let mut ids: [u8; 4] = [0; 4];
        for id in ids.iter_mut() {
            if let Some(part) = parts.next() {
                *id = part.parse::<u8>()?;
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
        let cid = "B.1.2.3.4".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 2, 3, 4]);
    }

    #[test]
    fn test_short_cid() {
        let cid = "B.1".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [1, 0, 0, 0]);
    }

    #[test]
    fn test_long_cid() {
        let cid = "B.11.11.11.11".parse::<CID>().unwrap();
        assert_eq!(cid.domain, Domain::Business);
        assert_eq!(cid.id, [11, 11, 11, 11]);
    }

    #[test]
    fn test_cid_invalid_domain() {
        assert!("A.1.2.3.4".parse::<CID>().is_err());
    }

    #[test]
    fn test_cid_sort() {
        let mut cids = vec![
            "S.1.1".parse().unwrap(),
            "S.1.11".parse().unwrap(),
            "S.1.2".parse().unwrap(),
            "P.3".parse().unwrap(),
        ];

        let correct_order: Vec<CID> = vec![
            "P.3".parse().unwrap(),
            "S.1.1".parse().unwrap(),
            "S.1.2".parse().unwrap(),
            "S.1.11".parse().unwrap(),
        ];

        cids.sort();

        assert_eq!(correct_order, cids);
    }
}
