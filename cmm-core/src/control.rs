use serde::{Deserialize, Serialize};

use crate::answer::Answer;
use std::ops::Not;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleControl {
    #[serde(flatten)]
    pub answer: Answer,
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    pub nist_only: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Control {
    title: String,
    remark: Option<String>,
    guidances: Vec<String>,
    comment: Option<String>,
    answer: Answer,
    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    nist_only: bool,
}

impl Control {
    pub fn new(
        title: String,
        remark: Option<String>,
        answer: Answer,
        comment: Option<String>,
        guidances: Vec<String>,
    ) -> Self {
        Self {
            title,
            remark,
            guidances,
            comment,
            answer,
            nist_only: false,
        }
    }
    pub fn guidance(&self) -> Option<&String> {
        self.answer
            .maturity_score()
            .or(self.answer.capability_score())
            .and_then(|score| self.guidances.get(score as usize))
    }

    pub fn guidances(&self) -> &Vec<String> {
        &self.guidances
    }

    pub fn set_guidances(&mut self, guidances: Vec<String>) {
        self.guidances = guidances;
    }

    pub fn answer(&self) -> &Answer {
        &self.answer
    }

    pub fn set_answer(&mut self, answer: Answer) {
        self.answer = answer;
    }

    pub fn title(&self) -> &String {
        &self.title
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

    pub fn to_simple(&self) -> SimpleControl {
        SimpleControl {
            answer: self.answer.clone(),
            comment: self.comment.clone(),
            nist_only: self.nist_only,
        }
    }
}
