use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{
    answer::Answer,
    cid::{CID, Domain},
    control::Control,
    schema::Schema,
    score::Score,
};

/// Only contains the soc-cmm values at its most simple form (CID->Control)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SOCData {
    #[serde(flatten)]
    controls: IndexMap<CID, Control>,
    notes: Option<String>,
    profile: IndexMap<String, String>,
}

impl SOCData {
    /// Only used by cmm-compat
    pub fn from_map(mut controls: HashMap<CID, Control>) -> Self {
        let mut indexmap = IndexMap::new();
        for (cid, control) in controls.drain() {
            indexmap.insert(cid, control);
        }
        indexmap.sort_keys();
        SOCData {
            controls: indexmap,
            notes: None,
            profile: IndexMap::new(),
        }
    }

    pub fn new(
        mut controls: IndexMap<CID, Control>,
        notes: Option<String>,
        profile: IndexMap<String, String>,
    ) -> Self {
        controls.sort_keys();
        SOCData {
            controls,
            notes,
            profile,
        }
    }

    pub fn controls_by_aspect(
        &self,
        domain: &Domain,
        aspect_id: u8,
    ) -> impl Iterator<Item = (&CID, &Control)> {
        self.controls
            .iter()
            .filter(move |(cid, _control)| cid.aspect_id() == aspect_id && cid.domain().eq(domain))
    }

    pub fn controls_by_domain(&self, domain: &Domain) -> impl Iterator<Item = (&CID, &Control)> {
        self.controls
            .iter()
            .filter(move |(cid, _control)| cid.domain().eq(domain))
    }

    /// Should not be public because user should modify controls over set_x(cid)
    fn control_mut(&mut self, cid: &CID) -> Option<&mut Control> {
        self.controls.get_mut(cid)
    }

    pub fn control(&self, cid: &CID) -> Option<&Control> {
        self.controls.get(cid)
    }

    pub fn remove_control(&mut self, cid: &CID) {
        self.controls.shift_remove(cid);
    }

    pub fn section_completeness(&self, cid: &CID) -> Score {
        let children: Vec<&Control> = self
            .controls_by_domain(&cid.domain())
            .filter(|(ctrl_id, ctrl)| {
                ctrl_id.is_child_of(cid) && ctrl.answer().type_eq(&Answer::Bool(true))
            })
            .map(|(_, ctrl)| ctrl)
            .collect();
        Score::new(
            children
                .iter()
                .filter(|ctrl| ctrl.answer().eq(&Answer::Bool(true)))
                .collect::<Vec<_>>()
                .len() as f64,
            children.len() as f64,
        )
    }

    pub fn set_answer(&mut self, cid: &CID, answer: Answer) {
        if let Some(control) = self.control_mut(cid) {
            control.set_answer(answer);
        }
    }

    pub fn set_comment(&mut self, cid: &CID, comment: Option<String>) {
        if let Some(control) = self.control_mut(cid) {
            control.set_comment(comment);
        }
    }

    pub fn toggle_bookmark(&mut self, cid: &CID) {
        if let Some(control) = self.control_mut(cid) {
            control.toggle_bookmark();
        }
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
    }

    pub fn has_pinned_items(&self) -> bool {
        self.controls
            .iter()
            .filter(|(_cid, control)| control.bookmark())
            .count()
            > 0
    }
}

impl From<&Schema> for SOCData {
    fn from(schema: &Schema) -> Self {
        SOCData {
            controls: schema
                .controls()
                .iter()
                .filter(|(_, control_schema)| Control::try_from(*control_schema).is_ok())
                .map(|(cid, control_schema)| (*cid, Control::try_from(control_schema).unwrap()))
                .collect(),
            notes: None,
            profile: IndexMap::new(),
        }
    }
}
