use std::fs::File;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap::Parser;
use keyword_dataset_rs::db::{ArxivDB, ArxivDBQueries};
use keyword_dataset_rs::err::AppError;
use keyword_dataset_rs::metadata::reader::ArxivMetadataIter;
use crate::args::Args;

pub mod args;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    let args = Args::parse();

    let file = File::open(&args.input)?;
    
    let mut db = ArxivDB::open(&args.db)?;
    
    // execute the DDL
    db.execute_ddl()?;
    
    let txn = db.txn()?;
    let queries = ArxivDBQueries::wrap(&txn);

    let start_date = Utc.with_ymd_and_hms(2020, 01, 01, 06, 00, 00).unwrap();
    let start_date = DateTime::<FixedOffset>::from(start_date);

    let metadata_iter = ArxivMetadataIter::new(file);
    metadata_iter.into_iter()
        .filter(|metadata| metadata.versions().first().is_some_and(|version| version.is_after(&start_date)))
        .filter(|metadata| metadata.categories().is_some_and(|categories| categories.contains("cs.")))
        .for_each(|metadata| {
            queries.insert_metadata(metadata).expect("failed to insert item into db");
        });
    
    txn.commit()?;
    
    println!("finished writing items to db");

    Ok(())
}
