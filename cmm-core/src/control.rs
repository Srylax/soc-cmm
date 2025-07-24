use serde::{Deserialize, Serialize};

use crate::answer::Answer;
use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Control {
    #[serde(flatten)]
    answer: Answer,
    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    bookmark: bool,

    comment: Option<String>,

    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    nist_only: bool,
}

impl Control {
    pub fn new(answer: Answer, comment: Option<String>) -> Self {
        Self {
            comment,
            answer,
            nist_only: false,
            bookmark: false,
        }
    }

    pub fn answer(&self) -> &Answer {
        &self.answer
    }

    pub fn set_answer(&mut self, answer: Answer) {
        self.answer = answer;
    }

    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }

    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }
    pub fn nist_only(&self) -> bool {
        self.nist_only
    }

    pub fn set_nist_only(&mut self, nist_only: bool) {
        self.nist_only = nist_only;
    }

    pub fn bookmark(&self) -> bool {
        self.bookmark
    }

    pub fn toggle_bookmark(&mut self) {
        self.bookmark = !self.bookmark;
    }
}
