use clap_derive::Subcommand;
use crate::subcommand::db::DBSubCommands;

pub mod db;

#[derive(Subcommand, Debug)]
pub enum AppSubCommands {
    /// DB-related features
    #[clap(subcommand)]
    DB(DBSubCommands),
}
