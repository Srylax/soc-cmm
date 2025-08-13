use serde::{Deserialize, Serialize};
use strum::VariantArray;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum QuestionType {
    YesNo,
    Text,
    /// yyyy-mm-dd
    Date,
    Number,
    Select { items: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, VariantArray)]
pub enum QuestionCategory {
    Personal,
    Organization
}

impl QuestionType {
    pub fn default_value (&self) -> String {
        match self {
            QuestionType::YesNo => String::from("No"),
            QuestionType::Select { items } => items.first().cloned().unwrap_or(String::new()),
            QuestionType::Date => String::from("2000-01-01"),
            QuestionType::Number => String::from("0"),
            QuestionType::Text => String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProfileQuestion {
    question: String,
    description: Option<String>,
    category: QuestionCategory,
    /// Short description of what the answer value represents
    short: String,

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
    
    pub fn short(&self) -> &str {
        &self.short
    }
    
    pub fn category(&self) -> &QuestionCategory {
        &self.category
    }
}
