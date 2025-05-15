use std::collections::HashMap;

use answer::{Answer, Detailed};
use strum::EnumCount;

pub mod answer;

pub enum Domain {
    People,
    Business,
    Process,
    Technology,
    Services,
}

pub type CID = String;

pub struct Aspect {
    domain: Domain,
    id: u8,
    controls: HashMap<CID, Control>,
}

impl Aspect {
    pub fn new(domain: Domain, id: u8, controls: HashMap<String, Control>) -> Self {
        Self {
            domain,
            id,
            controls,
        }
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

#[derive(Debug)]
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
