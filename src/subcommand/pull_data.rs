mod writer;

use std::fs::File;
use std::path::{Path, PathBuf};
use clap_derive::{Args, ValueEnum};
use indicatif::ProgressBar;
use log::{error, info};
use keyword_dataset_rs::db::ArxivDB;
use keyword_dataset_rs::db::pages::page_iter;
use keyword_dataset_rs::err::AppResult;
use keyword_dataset_rs::training::TrainingRecord;
use crate::subcommand::db::DBBaseArgs;
use crate::subcommand::pull_data::writer::{NdJsonOutputFormatter, OutputFormatter};

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    NDJSON
}

#[derive(Args, Debug)]
pub struct PullDataArgs {
    #[clap(flatten)]
    db: DBBaseArgs,
    /// path to output result to. If not provided, writes to STDOUT
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// format of output data
    #[arg(short, long, default_value = "ndjson")]
    format: OutputFormat,
}

pub(crate) fn pull_data(args: PullDataArgs) -> AppResult<()> {
    let db = match ArxivDB::open(&args.db.db) {
        Ok(db) => {
            info!("successfully connected to db {}", args.db.db.display());
            db
        }
        Err(err) => {
            error!("failed to open db: {}", err);
            return Err(err);
        }
    };

    db.execute_ddl()?;
    db.turn_off_synchronous()?;

    let queries = db.queries();

    let total_training_records = queries.count_training_data()?;
    info!("pulling {} training record(s)...", total_training_records);
    
    let pg = ProgressBar::new(total_training_records);

    let mut output_formatter = create_output_formatter(&args)?;
    for page in page_iter(total_training_records, 10) {
        let records = queries.select_training_data(page)?;
        for record in records {
            let record = TrainingRecord::from(record);
            output_formatter.write_record(record)?;
            pg.inc(1);
        }
    }
    pg.finish();

    info!("wrote all training data records to output file");

    Ok(())
}

fn create_output_formatter(args: &PullDataArgs) -> AppResult<Box<dyn OutputFormatter>> {
    match args.format {
        OutputFormat::NDJSON => {
            if let Some(output_path) = args.output.as_ref() {
                let file = open_output_file(output_path)?;
                Ok(NdJsonOutputFormatter::new(file).into_boxed_trait())
            } else {
                Ok(NdJsonOutputFormatter::new(std::io::stdout()).into_boxed_trait())
            }
        }
    }
}

fn open_output_file(path: &Path) -> AppResult<File> {
    match File::create_new(path) {
        Ok(file) => {
            info!("successfully opened output file {}", path.display());
            Ok(file)
        }
        Err(err) => {
            error!("failed to open output file: {}", err);
            Err(err.into())
        }
    }
}
