use std::path::PathBuf;
use clap_derive::Parser;

#[derive(Parser)]
pub struct Args {
    /// path to input kaggle file
    #[arg(short, long)]
    pub input: PathBuf,
}
