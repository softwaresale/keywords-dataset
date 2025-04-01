pub mod reader;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArxivVersion {
    version: String,
    created: String,
}

impl ArxivVersion {
    pub fn created_date(&self) -> chrono::ParseResult<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc2822(&self.created)
    }
    
    pub fn is_after(&self, date: &DateTime<FixedOffset>) -> bool {
        let Ok(created_date) = self.created_date() else {
            return false;
        };
        
        &created_date > date
    }
}

#[derive(Deserialize)]
pub struct ArxivMetadata {
    id: Option<String>,
    submitter: Option<String>,
    authors: Option<String>,
    title: Option<String>,
    comments: Option<String>,
    #[serde(alias = "journal-ref")]
    journal_ref: Option<String>,
    doi: Option<String>,
    categories: Option<String>,
    #[serde(alias = "abstract")]
    abstract_text: Option<String>,
    versions: Vec<ArxivVersion>,
}

impl ArxivMetadata {
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn submitter(&self) -> Option<&String> {
        self.submitter.as_ref()
    }

    pub fn authors(&self) -> Option<&String> {
        self.authors.as_ref()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn comments(&self) -> Option<&String> {
        self.comments.as_ref()
    }

    pub fn journal_ref(&self) -> Option<&String> {
        self.journal_ref.as_ref()
    }

    pub fn doi(&self) -> Option<&String> {
        self.doi.as_ref()
    }

    pub fn categories(&self) -> Option<&String> {
        self.categories.as_ref()
    }

    pub fn abstract_text(&self) -> Option<&String> {
        self.abstract_text.as_ref()
    }

    pub fn versions(&self) -> &Vec<ArxivVersion> {
        &self.versions
    }
}
