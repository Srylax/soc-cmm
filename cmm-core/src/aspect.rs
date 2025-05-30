use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{CID, CmmError, Control};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    pub(crate) controls: IndexMap<CID, Control>,
}

impl Aspect {
    pub fn try_from_map(controls: IndexMap<CID, Control>) -> crate::Result<Self> {
        // Get first Aspect ID
        let Some(prefix) = controls.keys().next().map(|cid| &cid[..1]) else {
            return Ok(Self::default());
        };
        // Ensure all other CIDs are equal to this Aspect ID
        if let Some(conflict) = controls.keys().find(|cid| !cid.starts_with(prefix)) {
            return Err(CmmError::MultipleAspects(
                prefix.to_owned(),
                conflict.clone(),
            ));
        }
        Ok(Self { controls })
    }

    pub fn maturity_factor(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().maturity_in_scope())
            .count() as u8
    }
    pub fn capability_factor(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().capability_in_scope())
            .count() as u8
    }
    pub fn maturity_total_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().maturity_in_scope())
            .flat_map(|cap| cap.answer().maturity_score())
            .sum()
    }
    pub fn capability_total_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().capability_in_scope())
            .flat_map(|cap| cap.answer().capability_score())
            .sum()
    }
    pub fn maturity_max_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().maturity_in_scope())
            .flat_map(|cap| cap.answer().max_score())
            .sum()
    }
    pub fn capability_max_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer().capability_in_scope())
            .flat_map(|cap| cap.answer().max_score())
            .sum()
    }
    pub fn maturity_final_score(&self) -> f64 {
        let factor = self.maturity_factor() as f64;
        let total_score = self.maturity_total_score() as f64;
        let max_score = self.maturity_max_score() as f64;

        (((total_score - factor) / (max_score - factor)) * 10000f64).round() / 100f64
    }

    pub fn capability_final_score(&self) -> f64 {
        let factor = self.capability_factor() as f64;
        let total_score = self.capability_total_score() as f64;
        let max_score = self.capability_max_score() as f64;

        (((total_score - factor) / (max_score - factor)) * 10000f64).round() / 100f64
    }

    pub fn maturity_score(&self) -> f64 {
        5f64 * (self.maturity_final_score() / 100f64)
    }

    pub fn capability_score(&self) -> f64 {
        5f64 * (self.capability_final_score() / 100f64)
    }
}
