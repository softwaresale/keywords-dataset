use log::debug;
use crate::content::ArxivPaperContent;
use crate::content::body::PaperBodyExtractor;
use crate::content::keyword::KeywordExtractor;
use crate::err::AppError;
use crate::fetch::PaperDownloader;

pub struct ContentExtractor {
    keyword_extractor: KeywordExtractor,
    paper_body_extractor: PaperBodyExtractor,
    paper_downloader: PaperDownloader,
}

impl ContentExtractor {
    pub fn new() -> Self {
        Self {
            keyword_extractor: KeywordExtractor::new(),
            paper_body_extractor: PaperBodyExtractor::new(),
            paper_downloader: PaperDownloader::new(),
        }
    }

    /// actually performs the relevant steps to fetch a paper and pull out content we want
    pub fn fetch_and_extract_content(&self, arxiv_id: String) -> ExtractResult<ArxivPaperContent> {
        // get the paper content
        let content = self.paper_downloader.fetch_paper_content(&arxiv_id)
            .map_err(error_mapper(&arxiv_id))?;
        debug!("processing {}: fetched content", arxiv_id);

        self.extract_content(arxiv_id, &content)
    }
    
    pub fn extract_content<StrT: Into<String>>(&self, arxiv_id: StrT, content: &str) -> ExtractResult<ArxivPaperContent> {
        let arxiv_id = arxiv_id.into();
        // get the keywords
        let keywords = self.keyword_extractor.extract_keywords(&content)
            .map_err(error_mapper(&arxiv_id))?;
        debug!("processing {}: extracted keywords", arxiv_id);

        // extract the paper content
        let content = self.paper_body_extractor.extract_body(&content)
            .map_err(error_mapper(&arxiv_id))?;
        debug!("processing {}: extracted paper body", arxiv_id);

        Ok(ArxivPaperContent {
            id: arxiv_id,
            paper_content: content,
            abstract_text: String::new(),
            keywords,
        })
    }
}

fn error_mapper(arxiv_id: &str) -> impl '_ + FnOnce(AppError) -> ExtractError {
    |err| {
        ExtractError {
            arxiv_id: arxiv_id.to_string(),
            err,
        }
    }
}

pub struct ExtractError {
    arxiv_id: String,
    err: AppError,
}

impl ExtractError {
    pub fn id(&self) -> &str {
        &self.arxiv_id
    }
    
    pub fn app_err(&self) -> &AppError {
        &self.err
    }
    
    pub fn into_app_error(self) -> AppError {
        self.err
    }
}

pub type ExtractResult<T> = Result<T, ExtractError>;
