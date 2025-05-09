pub enum Domain {
    People,
    Business,
    Process,
    Technology,
    Services,
}

// struct CID(Domain, u8, u8, u8);
//

// enum Detailed {
//     No = 1,
//     Partially = 2,
//     Averagely = 3,
//     Mostly = 4,
//     Fully = 5,
// }

#[derive(Clone, Copy)]
pub enum Satisfaction {
    No = 1,
    Somewhat = 2,
    Averagely = 3,
    Mostly = 4,
    Fully = 5,
}

pub enum Answer {
    Maturity(Satisfaction),
    Bool(bool),
    Float(f32),
}

impl Answer {
    pub fn in_scope(&self) -> bool {
        match self {
            Answer::Maturity(_) => true,
            Answer::Bool(_) => false,
            Answer::Float(_) => false,
        }
    }
    pub fn score(&self) -> Option<u8> {
        match self {
            Answer::Maturity(satisfaction) => Some(*satisfaction as u8),
            Answer::Bool(_) => None,
            Answer::Float(_) => None,
        }
    }
    pub fn max_score(&self) -> Option<u8> {
        match self {
            Answer::Maturity(_) => Some(5),
            Answer::Bool(_) => None,
            Answer::Float(_) => None,
        }
    }
}

pub struct Aspect {
    domain: Domain,
    id: u8,
    capabilities: Vec<CAP>,
}

impl Aspect {
    pub fn new(domain: Domain, id: u8, capabilities: Vec<CAP>) -> Self {
        Self {
            domain,
            id,
            capabilities,
        }
    }
    pub fn factor(&self) -> u8 {
        self.capabilities
            .iter()
            .filter(|cap| cap.answer.in_scope())
            .count() as u8
    }
    pub fn total_score(&self) -> u8 {
        self.capabilities
            .iter()
            .filter(|cap| cap.answer.in_scope())
            .flat_map(|cap| cap.answer.score())
            .sum()
    }
    pub fn max_score(&self) -> u8 {
        self.capabilities
            .iter()
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
