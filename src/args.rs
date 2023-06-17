use clap::{Parser, Args, Subcommand};

/// ViRust Memory Filler
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub commands: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    MemorySubcommand(MemoryArgs)
}

#[derive(Args, Debug)]
pub struct MemoryArgs {
    /// Gigas of RAM to be filled
    #[arg(short, long, default_value_t=0.0)]
    pub gigas: f64,

    /// Megas of RAM to be filled
    #[arg(short, long, default_value_t=0.0)]
    pub megas: f64,

    /// Kilobytes of RAM to be filled
    #[arg(short, long, default_value_t=100)]
    pub kilos: u64
}
