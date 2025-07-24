use control::Control;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::mem::discriminant;
use std::num::ParseIntError;
use std::str::ParseBoolError;

use answer::Answer;
use itertools::Itertools;
use strum::VariantArray;

pub mod answer;
pub mod cid;
pub mod control;
pub mod score;

use thiserror::Error;

use crate::cid::Domain;
use crate::cid::CID;

pub(crate) type Result<T> = std::result::Result<T, CmmError>;
#[derive(Error, Debug)]
pub enum CmmError {
    #[error("This aspect contains controls from multiple domains or aspects: ({0}) != ({1})")]
    MultipleAspects(CID, CID),
    #[error("Cannot extend an answer with mismatching type: {0:?} != {1:?}")]
    DiscriminantMismatch(Answer, Answer),
    #[error("Aspect with missing title found")]
    MissingAspectTitle,
    #[error("CID parsing error: No Domain in short format found")]
    CIDMissingDomain,
    #[error("CID parsing error: Cannot contain zero in the id")]
    CIDInvalidZero,
    #[error("CID parsing error: Identifier is malformed {0}")]
    CIDMalformed(#[from] ParseIntError),
    #[error(transparent)]
    StrumParseError(#[from] strum::ParseError),
    #[error(transparent)]
    ParseBoolError(#[from] ParseBoolError),
}

/// Only contains the soc-cmm values at its most simple form (CID->Control)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CMM {
    // History
    //
    //
    // struct Values {
    // controls: IndexMap<CID, Control>
    // custom_description: Option<String>
    // version: String
    // }
    //
    // struct Structure {
    // aspects: HashMap<Domain, Vec<Aspect>>
    // remarks: HashMap<CID, String>
    // guidances: IndexMap<CID, String>
    // }
    //
    // structure.aspects(domain) -> Vec<Aspect>
    // values1.controls(aspect) -> Vec<Control>
    // values2.controls(aspect) -> Vec<Control>
    // structure.guidance(control) -> String
    // structure.remark(control) -> String
    //
    controls: IndexMap<CID, Control>,
    notes: String,
}

impl CMM {
    /// Only used by cmm-compar
    pub fn new(mut controls: IndexMap<CID, Control>) -> Self {
        CMM {
            controls: IndexMap::new(),
            notes: String::new(),
        }
    }

    pub fn controls_by_aspect(&self, domain: &Domain, aspect_id: u8) -> impl Iterator<Item = (&CID, &Control)> {
        self.controls
            .iter()
            .filter(move |(cid, _control)| cid.aspect_id() == aspect_id && cid.domain().eq(domain))
    }

    pub fn controls_by_domain(&self, domain: &Domain) -> impl Iterator<Item = (&CID, &Control)> {
        self.controls
            .iter()
            .filter(move |(cid, _control)| cid.domain().eq(domain))
    }
}

/// let controls = soc_data.by_domain(domain) -> Vec<&Control>
/// soc_data.capability(controls) -> Score
/// soc_data.maturity(controls) -> Score

/// data.maturity_overall()
/// data.maturity_for_controls(controls)

#[derive(Debug, PartialEq, Clone)]
pub struct Score {
    score: f64,
    max: f64,
}

impl Score {
    pub fn from(score: f64, max: f64) -> Self {
        Self { score, max }
    }

    pub fn as_percentage(&self) -> f64 {
        self.score / self.max * 100.0
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

/// This is the soc-cmm schema and only contains Meta Information.
/// Changes will be made only between soc-cmm versions. The whole struct will be loaded at compile time.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Schema {
    /// AspectId = index+1, Aspects are only an index and a title
    aspects: HashMap<Domain, Vec<String>>,
    guidances: HashMap<CID, Vec<String>>,
    remarks: HashMap<CID, String>,
    titles: HashMap<CID, String>,
}

impl Schema {
    pub fn guidances(&self, cid: CID) -> Option<&Vec<String>> {
        self.guidances.get(&cid)
    }
}

impl CMM {
    pub fn control(&mut self, cid: &CID) -> Option<&mut Control> {
        self.controls.get_mut(cid)
    }

    pub fn set_answer(&mut self, cid: &CID, answer: Answer) {
        if let Some(control) = self.control(cid) {
            control.set_answer(answer);
        }
    }

    pub fn set_comment(&mut self, cid: &CID, comment: Option<String>) {
        if let Some(control) = self.control(cid) {
            control.set_comment(comment);
        }
    }

    pub fn toggle_bookmark(&mut self, cid: &CID) {
        if let Some(control) = self.control(cid) {
            control.toggle_bookmark();
        }
    }

    pub fn has_pinned_items(&self) -> bool {
        self.controls
            .iter()
            .filter(|(_cid, control)| control.bookmark())
            .count()
            > 0
    }

    // This is the only place where a CID with prefix is expected because it needs to be globally unique in the hashmap
    pub fn from_map(
        mut controls: HashMap<String, Control>,
        mut aspects: HashMap<String, String>,
    ) -> Result<Self> {
        let mut domains = IndexMap::new();
        for domain in Domain::VARIANTS {
            let domain_controls = controls.extract_if(|cid, _| cid.starts_with(domain.short()));
            let aspects = domain_controls
                .into_iter()
                .map(|(cid, control)| (cid[2..].to_owned(), control)) // Remove domain prefix
                // Ensure Aspect ordering
                .sorted_by_key(|(cid, _control)| {
                    cid.split('.')
                        .flat_map(|p| p.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .chunk_by(|(cid, _)| cid.chars().next().unwrap()) // Group by aspect ID
                .into_iter()
                .map(|(key, chunk)| {
                    (
                        chunk.collect::<IndexMap<_, _>>(),
                        aspects.remove(&format!("{}{key}", domain.short())),
                    )
                }) // Assemble into CID -> Control
                .map(|(map, title)| {
                    Aspect::try_from_map(map, title.ok_or(CmmError::MissingAspectTitle)?)
                })
                .collect::<Result<Vec<Aspect>>>()?;
            domains.insert(*domain, aspects);
        }
        Ok(Self {
            domains,
            custom_description: None,
        })
    }

    pub fn maturity_score_overall(&self) -> Score {
        self
            .controls_by_domain(domain)
    }

    pub fn maturity_score_domain(&self, domain: Domain) -> Score {}

    pub fn overall_max_maturity_score(&self) -> f64 {
        self.domains.len() as f64 * 5.0
    }

    pub fn aspect_maturity_score(&self, domain: &Domain) -> Option<f64> {
        let aspects = self.aspect(domain)?;
        let scores: Vec<f64> = aspects
            .iter()
            .map(|aspect| aspect.maturity_score())
            .collect();
        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }
    pub fn aspect_capability_score(&self, domain: &Domain) -> Option<f64> {
        let aspects = self.aspect(domain)?;
        let scores: Vec<f64> = aspects
            .iter()
            .map(|aspect| aspect.capability_score())
            .collect();
        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }

    pub fn as_simple(&self) -> IndexMap<Domain, IndexMap<CID, SimpleControl>> {
        self.domains
            .iter()
            .map(|(domain, aspects)| {
                (
                    *domain,
                    aspects
                        .iter()
                        .flat_map(|aspect| &aspect.controls)
                        .filter(|(_cid, control)| !matches!(control.answer(), Answer::Title))
                        .map(|(cid, control)| (cid.clone(), control.to_simple()))
                        .collect(),
                )
            })
            .collect()
    }

    pub fn extend_with_simple(
        &mut self,
        mut simple: IndexMap<Domain, IndexMap<CID, SimpleControl>>,
    ) -> Result<()> {
        for (domain, aspects) in self.domains.iter_mut() {
            let mut aspects: IndexMap<_, _> = aspects
                .iter_mut()
                .flat_map(|aspect| &mut aspect.controls)
                .collect();
            let Some(simple) = simple.shift_remove(domain) else {
                continue;
            };
            for (cid, simple_control) in simple {
                if let Some(control) = aspects.get_mut(&cid) {
                    if discriminant(control.answer()) != discriminant(&simple_control.answer) {
                        return Err(CmmError::DiscriminantMismatch(
                            control.answer().clone(),
                            simple_control.answer.clone(),
                        ));
                    }
                    control.set_answer(simple_control.answer);
                    control.set_comment(simple_control.comment);
                }
            }
        }
        Ok(())
    }

    pub fn custom_description(&self) -> &Option<String> {
        &self.custom_description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
