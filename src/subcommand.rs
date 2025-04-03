use clap_derive::Subcommand;
use crate::subcommand::db::DBSubCommands;
use crate::subcommand::extract::ExtractArgs;

pub mod db;
pub mod extract;

#[derive(Subcommand, Debug)]
pub enum AppSubCommands {
    /// DB-related features
    #[clap(subcommand)]
    DB(DBSubCommands),
    /// extract paper metadata
    Extract(ExtractArgs),
}
