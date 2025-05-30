use serde::{Deserialize, Serialize};

use crate::answer::Answer;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleControl {
    #[serde(flatten)]
    pub answer: Answer,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Control {
    title: String,
    remark: Option<String>,
    guidances: Vec<String>,
    comment: Option<String>,
    answer: Answer,
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
        }
    }
    pub fn guidance(&self) -> Option<&String> {
        self.answer
            .maturity_score()
            .or(self.answer.capability_score())
            .and_then(|score| self.guidances.get(score as usize))
    }

    pub fn set_guidances(&mut self, guidances: Vec<String>) {
        self.guidances = guidances;
    }

    pub fn set_answer(&mut self, answer: Answer) {
        self.answer = answer;
    }
    pub fn set_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }

    pub fn answer(&self) -> &Answer {
        &self.answer
    }
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }

    pub fn to_simple(&self) -> SimpleControl {
        SimpleControl {
            answer: self.answer.clone(),
            comment: self.comment.clone(),
        }
    }
}
