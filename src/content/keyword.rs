use crate::err::{AppError, AppResult};
use regex::Regex;

pub fn extract_keywords(contents: &str) -> AppResult<Vec<String>> {
    // find a block called keywords
    let keywords_header_start = Regex::new(r"\n\nKEYWORDS\n\n").unwrap();
    let intro_header = Regex::new(r"\n\n([\d.]+)\s*INTRODUCTION\n\n").unwrap();

    let keywords_section = keywords_header_start.find(&contents).ok_or(AppError::NoKeywords)?;
    let keywords_section_end = keywords_section.end();
    let refined_area = &contents[keywords_section_end..];

    let intro_match = intro_header.find(&refined_area).ok_or(AppError::NoKeywords)?;
    let intro_start = intro_match.start();

    let interesting_section = &refined_area[..intro_start];
    let keywords = interesting_section.split(", ")
        .map(|keyword| keyword.replace("\n", ""))
        .collect::<Vec<_>>();

    Ok(keywords)
}
