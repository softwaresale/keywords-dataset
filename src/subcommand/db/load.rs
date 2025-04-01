use std::fs::File;
use std::path::PathBuf;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap_derive::Args;
use regex::Regex;
use keyword_dataset_rs::db::{ArxivDB, ArxivDBQueries};
use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::metadata::reader::ArxivMetadataIter;
use crate::subcommand::db::DBBaseArgs;

#[derive(Args, Debug)]
pub struct DBLoadArgs {
    #[clap(flatten)]
    pub base: DBBaseArgs,
    /// path to the arxiv metadata file dump from kaggle
    pub metadata_file: PathBuf,
}

pub fn load_db(args: DBLoadArgs) -> AppResult<()> {
    
    let file = File::open(&args.metadata_file)?;
    let mut db = ArxivDB::open(&args.base.db)?;
    
    // execute the DDL
    db.execute_ddl()?;

    let txn = db.txn()?;
    let queries = ArxivDBQueries::wrap(&txn);

    let start_date = Utc.with_ymd_and_hms(2020, 01, 01, 06, 00, 00).unwrap();
    let start_date = DateTime::<FixedOffset>::from(start_date);

    // perform our filtration query and insert records into database
    let cs_category_filter = CsCategoryFilter::new();
    let metadata_iter = ArxivMetadataIter::new(file);
    let mut inserted_records = 0u64;
    metadata_iter.into_iter()
        .filter(|metadata| metadata.versions().first().is_some_and(|version| version.is_after(&start_date)))
        .filter(|metadata| metadata.categories().is_some_and(|categories| cs_category_filter.has_cs_categories(categories)))
        .for_each(|metadata| {
            queries.insert_arxiv_metadata(metadata).expect("failed to insert item into db");
            inserted_records += 1;
        });

    txn.commit()?;

    println!("finished writing {} item(s) to db", inserted_records);
    Ok(())
}

struct CsCategoryFilter {
    filter: Regex,
}

impl CsCategoryFilter {
    pub fn new() -> Self {
        Self {
            filter: Regex::new(r"^cs.\w\w$").unwrap()
        }
    }
    
    pub fn has_cs_categories(&self, categories: &str) -> bool {
        categories.split(" ")
            .any(|single| self.filter.is_match(single))
    }
}
