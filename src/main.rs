use crate::args::Args;
use crate::subcommand::db::handle_db_command;
use clap::Parser;
use keyword_dataset_rs::err::AppError;
use crate::subcommand::AppSubCommands;

mod args;
mod subcommand;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    let args = Args::parse();
    match args.command {
        AppSubCommands::DB(db_subcommand) => handle_db_command(db_subcommand)
    }
}
