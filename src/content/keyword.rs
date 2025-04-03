use log::debug;
use crate::err::{AppError, AppResult};
use regex::{Regex};
use crate::content::regexes::intro_header_regex_factory;

pub struct KeywordExtractor {
    keywords_header: Regex,
    intro_header: Regex,
    small_extractor: Regex,
    index_terms: Regex,
}

impl KeywordExtractor {
    pub fn new() -> Self {
        Self {
            keywords_header: Regex::new(r"\n\n *K[eE][yY][wW][oO][rR][dD][sS] *\n\n").unwrap(),
            intro_header: intro_header_regex_factory(),
            small_extractor: Regex::new(r"[Kk]eywords:?").unwrap(),
            index_terms: Regex::new(r"Index Terms—").unwrap()
        }
    }

    pub fn extract_keywords(&self, contents: &str) -> AppResult<Vec<String>> {
        // try each different approach
        self.extract_keywords_headers(contents)
            .or_else(|_| self.extract_keywords_headers_index_terms(contents))
            .or_else(|_| self.extract_keywords_small_label(contents))
    }

    pub fn extract_keywords_headers(&self, contents: &str) -> AppResult<Vec<String>> {
        debug!("using keyword extraction headers strategy");
        let keywords_section = self.keywords_header.find(&contents).ok_or(AppError::NoKeywords)?;
        let keywords_section_end = keywords_section.end();
        let refined_area = &contents[keywords_section_end..];

        let intro_match = self.intro_header.find(&refined_area).ok_or(AppError::NoKeywords)?;
        let intro_start = intro_match.start();

        let interesting_section = &refined_area[..intro_start];
        let keywords = Self::split_keywords(interesting_section);

        Ok(keywords)
    }

    pub fn extract_keywords_headers_index_terms(&self, contents: &str) -> AppResult<Vec<String>> {
        debug!("using keyword extraction headers (key terms) strategy");
        let keywords_section = self.index_terms.find(&contents).ok_or(AppError::NoKeywords)?;
        let keywords_section_end = keywords_section.end();
        let refined_area = &contents[keywords_section_end..];

        let intro_match = self.intro_header.find(&refined_area).ok_or(AppError::NoKeywords)?;
        let intro_start = intro_match.start();

        let interesting_section = &refined_area[..intro_start];
        let keywords = Self::split_keywords(interesting_section);

        Ok(keywords)
    }

    pub fn extract_keywords_small_label(&self, contents: &str) -> AppResult<Vec<String>> {
        debug!("using small headers strategy");
        // find the small extractor header
        let result = self.small_extractor.find(&contents)
            .ok_or(AppError::NoKeywords)?;

        let search_space = &contents[result.end()..];
        let keywords_str = take_until_double_newline(search_space);
        let keywords = Self::split_keywords(&keywords_str);
        
        Ok(keywords)
    }
    
    fn split_keywords(keyword_str: &str) -> Vec<String> {
        keyword_str
            .split(",")
            .map(|word| word.trim().to_string())
            .collect()
    } 
}

fn take_until_double_newline(content: &str) -> String {
    let mut output = String::new();
    let mut last_was_nl = false;
    for ch in content.chars() {
        
        // figure out how to interpret the next ch
        if ch == '\n' {
            if last_was_nl {
                break
            }
            
            last_was_nl = true
        } else {
            last_was_nl = false
        }
        
        output.push(ch);
    }
    
    output
}
