use std::fs::File;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap::Parser;
use keyword_dataset_rs::err::AppError;
use keyword_dataset_rs::metadata::reader::ArxivMetadataIter;
use crate::args::Args;

pub mod args;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    let args = Args::parse();

    let file = File::open(&args.input)?;

    let start_date = Utc.with_ymd_and_hms(2020, 01, 01, 06, 00, 00).unwrap();
    let start_date = DateTime::<FixedOffset>::from(start_date);

    let metadata_iter = ArxivMetadataIter::new(file);
    let ids = metadata_iter.into_iter()
        .filter(|metadata| metadata.versions().first().is_some_and(|version| version.is_after(&start_date)))
        .filter(|metadata| metadata.categories().is_some_and(|categories| categories.contains("cs.")))
        .filter_map(|meta| meta.id().cloned())
        .collect::<Vec<_>>();

    println!("got {} ids with cs.* categories published after {}", ids.len(), start_date);

    Ok(())
}
