use strum::{EnumCount, FromRepr};

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, FromRepr, EnumCount, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, FromRepr, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Answer {
    Satisfaction(Satisfaction),         // Maturity
    Detailed(Detailed),                 // Maturity
    DetailedOptional(DetailedOptional), // Capability
    Occurence(Occurence),               // Maturity
    Bool(bool),
    Any(String),
    None,
}

impl Answer {
    pub fn capability_in_scope(&self) -> bool {
        matches!(self, Answer::DetailedOptional(_))
            && *self != Answer::DetailedOptional(DetailedOptional::NotRequired)
    }
    pub fn maturity_in_scope(&self) -> bool {
        matches!(
            self,
            Answer::Satisfaction(_) | Answer::Occurence(_) | Answer::Detailed(_)
        )
    }
    pub fn maturity_score(&self) -> Option<u8> {
        match self {
            Answer::Satisfaction(satisfaction) => Some(*satisfaction as u8),
            Answer::Occurence(occurence) => Some(*occurence as u8),
            Answer::Detailed(detailed) => Some(*detailed as u8),
            _ => None,
        }
    }
    pub fn capability_score(&self) -> Option<u8> {
        match self {
            Answer::DetailedOptional(detailed_optional) => Some(*detailed_optional as u8),
            _ => None,
        }
    }
    pub fn max_score(&self) -> Option<u8> {
        match self {
            Answer::Satisfaction(_)
            | Answer::Occurence(_)
            | Answer::Detailed(_)
            | Answer::DetailedOptional(_) => Some(5),
            _ => None,
        }
    }
}
