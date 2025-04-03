use std::io::Read;
use ureq::{Agent, Body};
use ureq::http::Response;
use crate::err::{AppError, AppResult};
use crate::fetch::model::{GcsListObjectResponse, GcsObject};

pub struct GcsClient {
    agent: Agent,
}

impl GcsClient {
    pub fn new() -> Self {
        Self {
            agent: ureq::agent(),
        }
    }

    pub fn download_object_pdf(&self, object: GcsObject) -> AppResult<Vec<u8>> {
        let payload_size = object.size();
        if object.content_type != "application/pdf" {
            return Err(AppError::Other(format!("object '{}' content type is not PDF", object.id)))
        }

        let mut response = self.agent.get(object.media_link)
            .call()?;

        check_response_code(&response)?;

        let mut body_reader = response.body_mut().as_reader();
        let mut payload_buffer = Vec::<u8>::with_capacity(payload_size);
        body_reader.read_to_end(&mut payload_buffer)?;
        
        Ok(payload_buffer)
    }

    pub fn list_objects(&self, match_glob: String) -> AppResult<GcsListObjectResponse> {
        let mut response = self.agent.get("https://storage.googleapis.com/storage/v1/b/arxiv-dataset/o")
            .query("matchGlob", match_glob)
            .call()?;
        
        check_response_code(&response)?;

        let body_reader = response.body_mut().as_reader();
        let gcs_response = serde_json::from_reader::<_, GcsListObjectResponse>(body_reader)?;
        Ok(gcs_response)
    }
}

fn check_response_code(resp: &Response<Body>) -> AppResult<()> {
    if !resp.status().is_success() {
        return Err(AppError::HttpStatusError(resp.status()))
    }
    
    Ok(())
}
