use aspect::Aspect;
use control::Control;
use control::SimpleControl;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::mem::discriminant;
use strum::Display;

use answer::Answer;
use itertools::Itertools;
use strum::VariantArray;

pub mod answer;
pub mod aspect;
pub mod control;

use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, CmmError>;
#[derive(Error, Debug)]
pub enum CmmError {
    #[error("This aspect contains controls from multiple domains or aspects: ({0}) != ({1})")]
    MultipleAspects(CID, CID),
    #[error("Cannot extend an answer with mismatching type: {0:?} != {1:?}")]
    DiscriminantMismatch(Answer, Answer),
}

#[derive(
    VariantArray, Hash, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Display,
)]
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
    // This is the only place where a CID with prefix is expected because it needs to be globally unique in the hashmap
    pub fn from_map(mut controls: HashMap<String, Control>) -> Result<Self> {
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
                .chunk_by(|(cid, _)| cid.chars().next()) // Group by aspect ID
                .into_iter()
                .map(|(_key, chunk)| chunk.collect::<IndexMap<_, _>>()) // Assemble into CID -> Control
                .map(Aspect::try_from_map)
                .collect::<Result<Vec<Aspect>>>()?;
            domains.insert(*domain, aspects);
        }
        Ok(Self { domains })
    }

    pub fn aspect(&self, domain: &Domain) -> Option<&Vec<Aspect>> {
        self.domains.get(domain)
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
                        .filter(|(_cid, control)| !matches!(control.answer(), Answer::None))
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
}
