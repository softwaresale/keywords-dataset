mod model;
mod client;

use crate::err::{AppError, AppResult};
use crate::fetch::client::GcsClient;

// const MAX_BODY_SIZE: u64 = 10 * 1024 * 1024;

/// Downloading tool to ensure that we fairly download PDFs within the suggested rate limit of
/// 4 requests/second plus a 1-second sleep
pub struct PaperDownloader {
    gcs_client: GcsClient,
}

impl PaperDownloader {
    pub fn new() -> Self {
        Self {
            gcs_client: GcsClient::new(),
        }
    }

    /// downloads the respective arxiv paper using the id and fetches the text content of the paper
    pub fn fetch_paper_content(&self, arxiv_id: &str) -> AppResult<String> {
        // download the file
        let body_bytes = self.download_paper_pdf(arxiv_id)?;
        let pdf_content = pdf_extract::extract_text_from_mem(&body_bytes)?;
        Ok(pdf_content)
    }

    fn download_paper_pdf(&self, id: &str) -> AppResult<Vec<u8>> {

        let response = self.gcs_client.list_objects(glob_factory(id))?;
        let gcs_object = response.take_most_recent()
            .ok_or(AppError::NoBucketObject(id.to_string()))?;

        let contents = self.gcs_client.download_object_pdf(gcs_object)?;
        Ok(contents)
    }
}

fn glob_factory(arxiv_id: &str) -> String {

    let items = arxiv_id.split(".").collect::<Vec<_>>();
    assert_eq!(2, items.len(), "arxiv id should always be split into two parts");
    let year = items.first().expect("year should always be present");

    format!("arxiv/arxiv/pdf/{}/{}**", year, arxiv_id)
}
