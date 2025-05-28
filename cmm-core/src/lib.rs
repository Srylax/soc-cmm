use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use answer::Answer;
use itertools::Itertools;
use strum::VariantArray;

pub mod answer;

use thiserror::Error;

type Result<T> = std::result::Result<T, CmmError>;
#[derive(Error, Debug)]
pub enum CmmError {
    #[error("This aspect contains controls from multiple domains or aspects: ({0}) != ({1})")]
    MultipleAspects(CID, CID),
}

#[derive(VariantArray, Hash, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Domain {
    People,
    Business,
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
}

pub type CID = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct CMM {
    domains: IndexMap<Domain, Vec<Aspect>>,
}

impl CMM {
    pub fn from_map(mut controls: HashMap<CID, Control>) -> Result<Self> {
        let mut domains = IndexMap::new();
        for domain in Domain::VARIANTS {
            let domain_controls = controls.extract_if(|cid, _| cid.starts_with(domain.short()));
            let aspects = domain_controls
                .into_iter()
                // Ensure Aspect ordering
                .sorted_by_key(|(cid, _control)| {
                    cid[2..]
                        .split('.')
                        .flat_map(|p| p.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .chunk_by(|(cid, _)| cid.chars().nth(2)) // Group by aspect ID
                .into_iter()
                .map(|(_key, chunk)| chunk.collect::<IndexMap<_, _>>()) // Assemble into CID -> Control
                .map(Aspect::try_from_map)
                .collect::<Result<Vec<Aspect>>>()?;
            domains.insert(*domain, aspects);
        }
        Ok(Self { domains })
    }

    pub fn aspects(&self, domain: &Domain) -> Option<&Vec<Aspect>> {
        self.domains.get(domain)
    }

    pub fn to_simple(&self) -> IndexMap<&Domain, IndexMap<&CID, toml::Value>> {
        self.domains
            .iter()
            .map(|(domain, aspects)| {
                (
                    domain,
                    aspects
                        .iter()
                        .flat_map(|aspect| aspect.controls())
                        .map(|(cid, control)| (cid, control.answer.as_value()))
                        .collect(),
                )
            })
            .collect()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    controls: IndexMap<CID, Control>,
}

impl Aspect {
    pub fn try_from_map(controls: IndexMap<CID, Control>) -> Result<Self> {
        // Get first Aspect ID
        let Some(prefix) = controls.keys().next().map(|cid| &cid[..2]) else {
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

    pub fn controls(&self) -> &IndexMap<CID, Control> {
        &self.controls
    }

    pub fn maturity_factor(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.maturity_in_scope())
            .count() as u8
    }
    pub fn capability_factor(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.capability_in_scope())
            .count() as u8
    }
    pub fn maturity_total_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.maturity_in_scope())
            .flat_map(|cap| cap.answer.maturity_score())
            .sum()
    }
    pub fn capability_total_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.capability_in_scope())
            .flat_map(|cap| cap.answer.capability_score())
            .sum()
    }
    pub fn maturity_max_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.maturity_in_scope())
            .flat_map(|cap| cap.answer.max_score())
            .sum()
    }
    pub fn capability_max_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.capability_in_scope())
            .flat_map(|cap| cap.answer.max_score())
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
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Control {
    title: String,
    remark: Option<String>,
    guidances: Vec<String>,
    answer: Answer,
}

impl Control {
    pub fn new(
        title: String,
        remark: Option<String>,
        answer: Answer,
        guidances: Vec<String>,
    ) -> Self {
        Self {
            title,
            remark,
            guidances,
            answer,
        }
    }
    pub fn guidance(&self) -> Option<&String> {
        self.answer
            .maturity_score()
            .or(self.answer.capability_score())
            .and_then(|score| self.guidances.get(score as usize))
    }

    pub fn set_guidances(&mut self, guidances: Vec<String>) {
        self.guidances = guidances;
    }

    pub fn set_answer(&mut self, answer: Answer) {
        self.answer = answer;
    }

    pub fn answer(&self) -> &Answer {
        &self.answer
    }
}
