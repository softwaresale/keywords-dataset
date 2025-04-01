use std::fs::File;
use std::io::{BufReader, Read};
use keyword_dataset_rs::err::{AppError, AppResult};
use keyword_dataset_rs::keyword::{extract_keywords};

fn main() -> AppResult<()> {
    let bytes = {
        let file = File::open("/home/charlie/Programming/nlp-final-project/keyword-dataset-rs/examples/2503.20579")?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::<u8>::new();
        reader.read_to_end(&mut bytes)?;
        bytes
    };
    let content = pdf_extract::extract_text_from_mem(&bytes)?;
    
    let keywords = extract_keywords(&content)?;
    println!("keywords:");
    for keyword in keywords {
        println!("{}", keyword);
    }
    
    Ok(())
}
