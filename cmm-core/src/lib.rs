use std::collections::HashMap;

use strum::FromRepr;

pub enum Domain {
    People,
    Business,
    Process,
    Technology,
    Services,
}

#[derive(Clone, Copy, Debug, FromRepr)]
pub enum Satisfaction {
    No = 1,
    Somewhat = 2,
    Averagely = 3,
    Mostly = 4,
    Fully = 5,
}
impl Default for Satisfaction {
    fn default() -> Self {
        Self::No
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
pub enum Occurence {
    Never = 1,
    Sometimes = 2,
    Averagely = 3,
    Mostly = 4,
    Always = 5,
}
impl Default for Occurence {
    fn default() -> Self {
        Self::Never
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
pub enum Detailed {
    No = 1,
    Partially = 2,
    Averagely = 3,
    Mostly = 4,
    Fully = 5,
}
impl Default for Detailed {
    fn default() -> Self {
        Self::No
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
pub enum DetailedOptional {
    No = 1,
    Partially = 2,
    Averagely = 3,
    Mostly = 4,
    Fully = 5,
    NotRequired = 6,
}
impl Default for DetailedOptional {
    fn default() -> Self {
        Self::No
    }
}

#[derive(Debug)]
pub enum Answer {
    Satisfaction(Satisfaction),
    Detailed(Detailed),
    DetailedOptional(DetailedOptional),
    Occurence(Occurence),
    Bool(bool),
    Any(String),
    None,
}

impl Answer {
    pub fn in_scope(&self) -> bool {
        matches!(
            self,
            Answer::Satisfaction(_)
                | Answer::Occurence(_)
                | Answer::Detailed(_)
                | Answer::DetailedOptional(_)
        )
    }
    pub fn score(&self) -> Option<u8> {
        match self {
            Answer::Satisfaction(satisfaction) => Some(*satisfaction as u8),
            Answer::Occurence(occurence) => Some(*occurence as u8),
            Answer::Detailed(detailed) => Some(*detailed as u8),
            Answer::DetailedOptional(detailed_optional) => Some(*detailed_optional as u8),
            _ => None,
        }
    }
    pub fn max_score(&self) -> Option<u8> {
        match self {
            Answer::Satisfaction(_) | Answer::Occurence(_) | Answer::Detailed(_) => Some(5),
            Answer::DetailedOptional(_) => Some(6),
            _ => None,
        }
    }
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
