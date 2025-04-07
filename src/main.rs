use crate::args::Args;
use crate::subcommand::db::handle_db_command;
use crate::subcommand::extract::extract_and_save_contents;
use crate::subcommand::pull_data::pull_data;
use crate::subcommand::AppSubCommands;
use clap::Parser;
use keyword_dataset_rs::err::AppError;
use log::LevelFilter;

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
        AppSubCommands::PullTraining(args) => pull_data(args),
    }
}
