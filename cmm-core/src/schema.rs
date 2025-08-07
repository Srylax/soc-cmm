/// {
///      "aspects": {
///          "People": {
///              ["Title"],
///              ["Title"],
///          }
///      },
///      "control_schemas": {
///          "P.1.0": {
///              "guidances": ["Str"],
///              "remarks": "",
///              "title": "",
///          }
///      }
/// }
use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::ops::Not;

use crate::{answer::Answer, cid::{Domain, CID}};

/// This is the soc-cmm schema and only contains Meta Information.
/// Changes will be made only between soc-cmm versions. The whole struct will be loaded at compile time.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Schema {
    /// AspectId = index+1, Aspects are only an index and a title
    aspects: HashMap<Domain, Vec<String>>,
    control_schemas: HashMap<CID, ControlSchema>,
}

impl Schema {
    pub fn aspects(&self, domain: &Domain) -> Vec<&String> {
        self.aspects
            .get(domain)
            .map(|vec| vec.iter().collect())
            .unwrap_or_default()
    }

    pub fn control_schema(&self, cid: &CID) -> Option<&ControlSchema> {
        self.control_schemas.get(cid)
    }

    pub fn controls_by_aspect(
        &self,
        domain: &Domain,
        aspect_id: u8
    ) -> impl Iterator<Item = (&CID, &ControlSchema)> {
        self.control_schemas
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .filter(move |(cid, _control)| cid.aspect_id() == aspect_id && cid.domain().eq(domain))
    }

    pub fn controls(&self) -> &HashMap<CID, ControlSchema> {
        &self.control_schemas
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ControlType {
    Satisfaction,
    Detailed,
    DetailedOptional,
    Occurence,               // Maturity
    Bool,
    Any,
    Title,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ControlSchema {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    guidances: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    remarks: Option<String>,

    title: String,
    control_type: ControlType,

    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    nist_only: bool
}

impl ControlSchema {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn guidances(&self) -> &Vec<String> {
        &self.guidances
    }

    pub fn remarks(&self) -> &Option<String> {
        &self.remarks
    }

    pub fn control_type(&self) -> &ControlType {
        &self.control_type
    }

    pub fn nist_only(&self) -> bool {
        self.nist_only
    }
}
