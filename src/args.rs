use clap_derive::Parser;
use crate::subcommand::AppSubCommands;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: AppSubCommands,
}
