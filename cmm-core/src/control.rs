use serde::{Deserialize, Serialize};

use crate::{answer::Answer, schema::ControlSchema};
use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Control {
    #[serde(flatten)]
    answer: Answer,

    #[serde(skip_serializing_if = "<&bool>::not")]
    #[serde(default)]
    bookmark: bool,

    comment: Option<String>,
}

impl Control {
    pub fn new(answer: Answer, comment: Option<String>) -> Self {
        Self {
            comment,
            answer,
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

    pub fn bookmark(&self) -> bool {
        self.bookmark
    }

    pub fn toggle_bookmark(&mut self) {
        self.bookmark = !self.bookmark;
    }

    pub fn is_default(&self) -> bool {
        self.answer.is_default() && self.comment.is_none() && !self.bookmark
    }
}

impl From<&ControlSchema> for Control {
    fn from(value: &ControlSchema) -> Self {
        Self {
            answer: Answer::from(value.control_type()),
            bookmark: false,
            comment: None,
        }
    }
}
