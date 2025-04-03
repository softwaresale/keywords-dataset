use std::ops::Deref;
use std::sync::{Arc};
use crate::subcommand::db::DBBaseArgs;
use clap_derive::Args;
use log::{debug, error, info};
use threadpool::ThreadPool;
use keyword_dataset_rs::content::ArxivPaperContent;
use keyword_dataset_rs::db::{ArxivDB, ArxivDBQueries};
use keyword_dataset_rs::db::pages::page_iter;
use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::extraction::{ContentExtractor, ExtractResult};

#[derive(Args, Debug)]
pub struct ExtractArgs {
    #[clap(flatten)]
    db: DBBaseArgs,
    /// optionally provide how many papers you want to provide extraction for
    #[arg(short, long)]
    count: Option<u64>,
}

pub fn extract_and_save_contents(args: ExtractArgs) -> AppResult<()> {

    let mut db = match ArxivDB::open(&args.db.db) {
        Ok(db) => {
            info!("successfully connected to db {}", args.db.db.display());
            db
        }
        Err(err) => {
            error!("failed to connect to db: {}", err);
            return Err(err);
        }
    };
    
    // execute the DDL and turn off synchronous
    db.execute_ddl()?;
    db.turn_off_synchronous()?;

    let txn = db.txn()?;
    let queries = ArxivDBQueries::wrap(txn.deref());

    let (total_ids, is_sample) = if let Some(sample_size) = args.count {
        info!("going to process random sample of size {}", sample_size);
        (sample_size, true)
    } else {
        let total_ids = queries.count_arxiv_ids()?;
        info!("going to process all {} id(s)", total_ids);
        (total_ids, false)
    };

    let pool = threadpool::Builder::new()
        .thread_name("extractor-thread-".to_string())
        .num_threads(std::thread::available_parallelism()?.get())
        .build();
    let extractor = Arc::new(ContentExtractor::new());

    if is_sample {
        process_sample(&queries, extractor, pool, total_ids)?;
    } else {
        process_all(&queries, extractor, pool, total_ids)?;
    }

    txn.commit()?;

    info!("successfully committed transaction to update paper contents");

    Ok(())
}

fn process_all(
    queries: &ArxivDBQueries,
    extractor: Arc<ContentExtractor>,
    pool: ThreadPool,
    total_ids: u64
) -> AppResult<()> {
    for page in page_iter(total_ids, 10) {
        info!("processing page {}", page);

        let ids = match queries.select_arxiv_ids(page) {
            Ok(ids) => {
                debug!("successfully got ids from db");
                ids
            }
            Err(err) => {
                error!("error encountered while fetching ids from db: {}", err);
                return Err(err);
            }
        };
        
        let contents = extract_paper_contents(extractor.clone(), &pool, ids)?;

        // wait for all content to be extracted
        info!("waiting on content extractors...");
        pool.join();
        info!("all done");

        for item in contents {
            match item {
                Ok(content) => {
                    info!("inserting content for {}", &content.id);
                    queries.update_keywords_and_content(content)?;
                }
                Err(err) => {
                    error!("error while extracting content from {}: {}", err.id(), err.app_err());
                }
            }
        }
    }

    Ok(())
}

fn process_sample(
    queries: &ArxivDBQueries,
    extractor: Arc<ContentExtractor>,
    pool: ThreadPool,
    sample_size: u64
) -> AppResult<()> {
    let ids = queries.sample_arxiv_ids(sample_size)?;

    let results = extract_paper_contents(extractor, &pool, ids)?;
    
    for item in results {
        match item {
            Ok(content) => {
                info!("inserting content for {}", &content.id);
                // update the status
                queries.insert_extraction_result(&content.id, None)?;
                // insert the content
                queries.update_keywords_and_content(content)?;
            }
            Err(err) => {
                error!("error while extracting content from {}: {}", err.id(), err.app_err());
                // just log that we had some kind of error
                queries.insert_extraction_result("", Some(err))?;
            }
        }
    }
    
    Ok(())
}

fn extract_paper_contents(
    extractor: Arc<ContentExtractor>,
    pool: &ThreadPool,
    ids: Vec<String>,
) -> AppResult<Vec<ExtractResult<ArxivPaperContent>>> {
    let (sender, recv) = std::sync::mpsc::channel::<ExtractResult<ArxivPaperContent>>();

    for id in ids {
        let extractor = extractor.clone();
        let send = sender.clone();
        pool.execute(move || {
            debug!("starting to extract content for {}", id);
            let result = extractor.fetch_and_extract_content(id);
            send.send(result).unwrap();
        })
    }

    drop(sender);

    pool.join();

    let items = recv.iter().collect::<Vec<_>>();
    Ok(items)
}
