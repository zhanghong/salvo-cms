use clap::Parser;

mod server;
pub use server::start as server_start; 

use crate::enums::CommandEnum;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandArgs {
    #[clap(subcommand)]
    pub name: Option<CommandEnum>,
}