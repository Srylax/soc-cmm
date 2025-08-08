use std::num::ParseIntError;
use std::str::ParseBoolError;

use answer::Answer;

pub mod answer;
pub mod cid;
pub mod control;
pub mod data;
pub mod schema;
pub mod score;

use thiserror::Error;

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
