use crate::subcommand::db::DBBaseArgs;
use clap_derive::Args;
use keyword_dataset_rs::content::ArxivPaperContent;
use keyword_dataset_rs::db::pages::page_iter;
use keyword_dataset_rs::db::{ArxivDB, ArxivDBQueries};
use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::extraction::{ContentExtractor, ExtractResult};
use log::{debug, error, info};
use std::ops::Deref;
use std::sync::Arc;
use threadpool::ThreadPool;

#[derive(Args, Debug)]
pub struct ExtractArgs {
    #[clap(flatten)]
    db: DBBaseArgs,
    /// optionally provide how many papers you want to provide extraction for
    #[arg(short, long)]
    count: Option<u64>,
    /// used in conjunction with count. If true, only sample from records without corresponding
    /// extraction_result
    #[arg(short, long, default_value_t = false)]
    unique: bool,
    /// how many threads are available. 0 will use available parallelism
    #[arg(short = 'j', long, default_value_t = 0usize)]
    parallelism: usize,
}

impl ExtractArgs {
    pub fn parallelism(&self) -> usize {
        if self.parallelism == 0 {
            return std::thread::available_parallelism().unwrap().get();
        }

        self.parallelism
    }
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
        .num_threads(args.parallelism())
        .build();
    let extractor = Arc::new(ContentExtractor::new());

    if is_sample {
        process_sample(
            &queries,
            extractor,
            pool,
            total_ids,
            args.unique,
            args.parallelism(),
        )?;
    } else {
        process_all(&queries, extractor, pool, total_ids)?;
    }

    info!("starting to commit extraction results...");
    txn.commit()?;

    info!("successfully committed transaction to update paper contents");

    Ok(())
}

fn process_all(
    queries: &ArxivDBQueries,
    extractor: Arc<ContentExtractor>,
    pool: ThreadPool,
    total_ids: u64,
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
                    error!(
                        "error while extracting content from {}: {}",
                        err.id(),
                        err.app_err()
                    );
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
    sample_size: u64,
    unique: bool,
    batch_size: usize,
) -> AppResult<()> {
    let ids = if unique {
        queries.sample_arxiv_ids_unprocessed(sample_size)
    } else {
        queries.sample_arxiv_ids(sample_size)
    }?;

    // process ids in batches
    for batch in ids.chunks(batch_size) {
        let id_batch = batch.to_vec();
        let results = extract_paper_contents(extractor.clone(), &pool, id_batch)?;
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
                    error!(
                        "error while extracting content from {}: {}",
                        err.id(),
                        err.app_err()
                    );
                    // just log that we had some kind of error
                    queries.insert_extraction_result("", Some(err))?;
                }
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
