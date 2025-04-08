use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::keyword::extract_keywords_from_pdf_file;

fn main() -> AppResult<()> {
    extract_keywords_from_pdf_file("/home/charlie/Programming/nlp-final-project/keyword-dataset-rs/examples/2503.20579")?;
    
    Ok(())
}
