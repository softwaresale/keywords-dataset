use clap_derive::Subcommand;
use crate::subcommand::db::DBSubCommands;
use crate::subcommand::extract::ExtractArgs;
use crate::subcommand::pull_data::PullDataArgs;

pub(crate) mod db;
pub(crate) mod extract;
pub(crate) mod pull_data;

#[derive(Subcommand, Debug)]
pub enum AppSubCommands {
    /// DB-related features
    #[clap(subcommand)]
    DB(DBSubCommands),
    /// extract paper metadata
    Extract(ExtractArgs),
    /// pulls training data from the DB
    PullTraining(PullDataArgs),
}
