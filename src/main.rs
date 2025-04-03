use crate::args::Args;
use crate::subcommand::db::handle_db_command;
use clap::Parser;
use log::LevelFilter;
use keyword_dataset_rs::err::AppError;
use crate::subcommand::{AppSubCommands};
use crate::subcommand::extract::extract_and_save_contents;

mod args;
mod subcommand;

fn main() -> Result<(), AppError> {

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let args = Args::parse();
    match args.command {
        AppSubCommands::DB(db_subcommand) => handle_db_command(db_subcommand),
        AppSubCommands::Extract(extract_args) => extract_and_save_contents(extract_args),
    }
}
