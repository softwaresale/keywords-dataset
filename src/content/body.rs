use regex::Regex;
use crate::content::regexes::intro_header_regex_factory;
use crate::err::{AppError, AppResult};

pub struct PaperBodyExtractor {
    intro_header: Regex,
    references_header: Regex,
    references_header_permissive: Regex,
}

impl PaperBodyExtractor {
    pub fn new() -> Self {
        Self {
            intro_header: intro_header_regex_factory(),
            references_header: Regex::new(r"\n\nR[eE][fF][eE][rR][eE][nN][cC][eE][sS] *\n\n").unwrap(),
            references_header_permissive: Regex::new(r"\n\n?(.+)R[eE][fF][eE][rR][eE][nN][cC][eE][sS] *\n\n").unwrap(),
        }
    }
    
    pub fn extract_body(&self, content: &str) -> AppResult<String> {
        // find where the intro starts
        let intro_match = self.intro_header.find(content)
            .ok_or(AppError::MissingSection("INTRODUCTION".to_string()))?;
        
        // get the starting bound
        let content_start = intro_match.start();
        
        let references_match = self.references_header.find_at(content, content_start)
            .or_else(|| self.references_header_permissive.find_at(content, content_start))
            .ok_or(AppError::MissingSection("REFERENCES".to_string()))?;
        
        let content_end = references_match.start();
        
        let content = String::from(&content[content_start..content_end]);
        Ok(content)
    }
}
