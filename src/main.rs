mod args;
mod subcommands;

use clap::Parser;
use log::{info, error};

use args::{Arguments, Commands::Memory};

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    pretty_env_logger::init();

    info!("Arguments: {:?}", args);

    match args.commands {
        None => error!("Default GUI Not Implemented"),
        Some(command) => match command {
            Memory(args) => subcommands::memory::fill_memory(args).await
        }
    };
}
