mod load;

use std::path::PathBuf;
use clap_derive::{Args, Subcommand};
use keyword_dataset_rs::err::AppResult;
use crate::subcommand::db::load::{load_db, DBLoadArgs};

#[derive(Subcommand, Debug)]
pub enum DBSubCommands {
    /// load data into a new database
    Load(DBLoadArgs),
}

#[derive(Args, Debug)]
pub struct DBBaseArgs {
    /// path to the SQLite metadata db that we want to operate on
    #[arg(short, long)]
    pub db: PathBuf,
}

pub fn handle_db_command(cmd: DBSubCommands) -> AppResult<()> {
    match cmd {
        DBSubCommands::Load(args) => load_db(args),
    }
}
