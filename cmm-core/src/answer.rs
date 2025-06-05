use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;
use strum::VariantNames;
use strum::{EnumCount, FromRepr};

#[derive(
    Clone,
    Copy,
    Debug,
    FromRepr,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::Display,
    VariantNames,
    EnumString,
)]
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

#[derive(
    Clone,
    Copy,
    Debug,
    FromRepr,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::Display,
    VariantNames,
    EnumString,
)]
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

#[derive(
    Clone,
    Copy,
    Debug,
    FromRepr,
    EnumCount,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::Display,
    VariantNames,
    EnumString,
)]
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

#[derive(
    Clone,
    Copy,
    Debug,
    FromRepr,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::Display,
    VariantNames,
    EnumString,
)]
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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "answer")]
pub enum Answer {
    Satisfaction(Satisfaction),         // Maturity
    Detailed(Detailed),                 // Maturity
    DetailedOptional(DetailedOptional), // Capability
    Occurence(Occurence),               // Maturity
    Bool(bool),
    Any(String),
    Title,
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
    pub const fn variants(&self) -> &'static [&'static str] {
        match self {
            Answer::Satisfaction(_) => Satisfaction::VARIANTS,
            Answer::Detailed(_) => Detailed::VARIANTS,
            Answer::DetailedOptional(_) => DetailedOptional::VARIANTS,
            Answer::Occurence(_) => Occurence::VARIANTS,
            Answer::Bool(_) => &["true", "false"],
            Answer::Any(_) => &[],
            Answer::Title => &[],
        }
    }
    pub fn variant_eq(&self, variant: &str) -> bool {
        match self {
            Answer::Satisfaction(satisfaction) => satisfaction.to_string() == variant,
            Answer::Detailed(detailed) => detailed.to_string() == variant,
            Answer::DetailedOptional(detailed_optional) => detailed_optional.to_string() == variant,
            Answer::Occurence(occurence) => occurence.to_string() == variant,
            Answer::Bool(boolean) => boolean.to_string() == variant,
            Answer::Any(_) => false,
            Answer::Title => false,
        }
    }

    pub fn extend_from_variant(&self, variant: &str) -> crate::Result<Answer> {
        Ok(match self {
            Answer::Satisfaction(_) => Answer::Satisfaction(Satisfaction::from_str(variant)?),
            Answer::Detailed(_) => Answer::Detailed(Detailed::from_str(variant)?),
            Answer::DetailedOptional(_) => {
                Answer::DetailedOptional(DetailedOptional::from_str(variant)?)
            }
            Answer::Occurence(_) => Answer::Occurence(Occurence::from_str(variant)?),
            Answer::Bool(_) => Answer::Bool(bool::from_str(variant)?),
            Answer::Any(any) => Answer::Any(any.clone()),
            Answer::Title => Answer::Title,
        })
    }

    pub fn as_value(&self) -> String {
        return match self {
            Answer::Satisfaction(satisfaction) => satisfaction.to_string(),
            Answer::Detailed(detailed) => detailed.to_string(),
            Answer::DetailedOptional(detailed_optional) => detailed_optional.to_string(),
            Answer::Occurence(occurence) => occurence.to_string(),
            Answer::Bool(boolean) => boolean.to_string(),
            Answer::Any(any) => any.to_string(),
            Answer::Title => String::new(),
        };
    }
}
