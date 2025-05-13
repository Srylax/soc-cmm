use std::collections::HashMap;

use answer::Answer;
use strum::FromRepr;

mod answer;

pub enum Domain {
    People,
    Business,
    Process,
    Technology,
    Services,
}

pub struct Aspect {
    domain: Domain,
    id: u8,
    capabilities: HashMap<String, CAP>,
}

impl Aspect {
    pub fn new(domain: Domain, id: u8, capabilities: HashMap<String, CAP>) -> Self {
        Self {
            domain,
            id,
            capabilities,
        }
    }
    pub fn factor(&self) -> u8 {
        self.capabilities
            .values()
            .filter(|cap| cap.answer.in_scope())
            .count() as u8
    }
    pub fn total_score(&self) -> u8 {
        self.capabilities
            .values()
            .filter(|cap| cap.answer.in_scope())
            .flat_map(|cap| cap.answer.score())
            .sum()
    }
    pub fn max_score(&self) -> u8 {
        self.capabilities
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

pub struct CAP {
    answer: Answer,
}

impl CAP {
    pub fn new(answer: Answer) -> Self {
        Self { answer }
    }
}
