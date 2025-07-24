use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use strum::VariantArray;

use crate::{
    answer::Answer,
    cid::{CID, Domain},
    control::Control,
    score::{Score, ScoreCalculator},
};

/// Only contains the soc-cmm values at its most simple form (CID->Control)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SOCData {
    #[serde(flatten)]
    controls: IndexMap<CID, Control>,
    notes: Option<String>,
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
        }
    }

    pub fn new(mut controls: IndexMap<CID, Control>, notes: Option<String>) -> Self {
        controls.sort_keys();
        SOCData { controls, notes }
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

    pub fn maturity_score_overall(&self) -> Score {
        let mut score = 0.0;
        for domain in Domain::VARIANTS {
            score += self
                .controls_by_domain(domain)
                .map(|(_cid, control)| control)
                .maturity_score()
                .score();
        }
        Score::new(score, Domain::VARIANTS.len() as f64 * 5.0)
    }

    pub fn capability_score_by_domain(&self, domain: &Domain) -> Score {
        self.controls_by_domain(domain)
            .map(|(_cid, control)| control)
            .capability_score()
    }

    pub fn maturity_score_by_domain(&self, domain: &Domain) -> Score {
        self.controls_by_domain(domain)
            .map(|(_cid, control)| control)
            .capability_score()
    }
}
