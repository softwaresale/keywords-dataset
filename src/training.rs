use serde::Serialize;
use crate::content::{ArxivPaperContentEntity};

#[derive(Serialize, Debug)]
pub struct TrainingRecord {
    pub arxiv_id: String,
    pub content: String,
    pub abstract_content: String,
    pub keywords: Vec<String>,
}

impl TrainingRecord {
    pub fn parse_keywords(keywords: String) -> Vec<String> {
        keywords
            .split(",")
            .map(|term| term.trim())
            .map(strip_internal_newlines)
            .collect()
    }
}

fn strip_internal_newlines(term: &str) -> String {
    term.chars()
        .filter(|ch| *ch != '\n')
        .collect()
}

impl From<ArxivPaperContentEntity> for TrainingRecord {
    fn from(value: ArxivPaperContentEntity) -> Self {
        Self {
            arxiv_id: value.id,
            content: value.paper_content,
            abstract_content: value.abstract_text,
            keywords: TrainingRecord::parse_keywords(value.keywords),
        }
    }
}
