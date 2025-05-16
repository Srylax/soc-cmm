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

#[derive(VariantArray, Hash, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug)]
pub struct CMM {
    domains: HashMap<Domain, Vec<Aspect>>,
}

impl CMM {
    pub fn from_map(mut controls: HashMap<CID, Control>) -> Result<Self> {
        let mut domains = HashMap::new();
        for domain in Domain::VARIANTS {
            let domain_controls = controls.extract_if(|cid, _| cid.starts_with(domain.short()));
            let aspects = domain_controls
                .into_iter()
                .sorted_by(|a, b| Ord::cmp(&a.0, &b.0)) // Ensure Aspect ordering
                .chunk_by(|(cid, _)| cid.chars().nth(2)) // Group by aspect ID
                .into_iter()
                .map(|(_key, chunk)| chunk.collect::<HashMap<_, _>>()) // Assemble into CID -> Control
                .map(Aspect::try_from_map)
                .collect::<Result<Vec<Aspect>>>()?;
            domains.insert(*domain, aspects);
        }
        Ok(Self { domains })
    }

    pub fn aspects(&self, domain: &Domain) -> Option<&Vec<Aspect>> {
        self.domains.get(domain)
    }
}

#[derive(Default, Debug)]
pub struct Aspect {
    controls: HashMap<CID, Control>,
}

impl Aspect {
    pub fn try_from_map(controls: HashMap<CID, Control>) -> Result<Self> {
        let Some(prefix) = controls.keys().next().map(|cid| &cid[..2]) else {
            return Ok(Self::default());
        };
        if let Some(conflict) = controls.keys().find(|cid| !cid.starts_with(prefix)) {
            return Err(CmmError::MultipleAspects(
                prefix.to_owned(),
                conflict.clone(),
            ));
        }
        Ok(Self { controls })
    }

    pub fn factor(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.in_scope())
            .count() as u8
    }
    pub fn total_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.in_scope())
            .flat_map(|cap| cap.answer.score())
            .sum()
    }
    pub fn max_score(&self) -> u8 {
        self.controls
            .values()
            .filter(|cap| cap.answer.in_scope())
            .flat_map(|cap| cap.answer.max_score())
            .sum()
    }
    pub fn final_score(&self) -> f64 {
        let factor = self.factor() as f64;
        let total_score = self.total_score() as f64;
        let max_score = self.max_score() as f64;

        (((total_score - factor) / (max_score - factor)) * 10000f64).round() / 100f64
    }
}

#[derive(Debug, PartialEq, Eq)]
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
            .score()
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
