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

use serde::{Deserialize, Serialize};

use crate::cid::{CID, Domain};

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
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ControlSchema {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    guidances: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remarks: Option<String>,
    title: String,
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
}
