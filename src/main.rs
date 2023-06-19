mod args;
mod subcommands;

use clap::Parser;
use log::info;

use args::{Arguments, Commands::{Memory, Gui}};

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    pretty_env_logger::init();

    info!("Arguments: {:?}", args);

    match args.commands {
        None => subcommands::gui::start_gui(),
        Some(command) => match command {
            Memory(args) => subcommands::memory::fill_memory(args).await,
            Gui => subcommands::gui::start_gui()
        }
    };
}
