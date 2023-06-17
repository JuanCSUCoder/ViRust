mod args;
mod subcommands;

use std::{collections::LinkedList, sync::{Arc, Mutex}};

use clap::Parser;
use log::{info, trace, error};

use args::{Arguments, Commands::MemorySubcommand};

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    pretty_env_logger::init();

    info!("Arguments: {:?}", args);

    match args.commands {
        None => error!("Default GUI Not Implemented"),
        Some(command) => match command {
            MemorySubcommand(args) => subcommands::memory::fill_memory(args).await
        }
    };
}
