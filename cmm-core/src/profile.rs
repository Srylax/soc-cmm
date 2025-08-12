use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum QuestionType {
    Bool,
    Text,
    Date,
    Number,
    Select { items: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProfileQuestion {
    question: String,
    description: Option<String>,

    #[serde(flatten)]
    question_type: QuestionType,
}

impl ProfileQuestion {
    pub fn question(&self) -> &String {
        &self.question
    }
    
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    
    pub fn question_type(&self) -> &QuestionType {
        &self.question_type
    }
}
