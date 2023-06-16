use std::{collections::LinkedList, io::{self, Read}};

use clap::Parser;
use log::info;

/// ViRust Memory Filler
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Arguments {
    /// Gigas of RAM to be filled
    #[arg(short, long, default_value_t=0.0)]
    gigas: f64,

    /// Megas of RAM to be filled
    #[arg(short, long, default_value_t=0.0)]
    megas: f64,

    /// Kilobytes of RAM to be filled
    #[arg(short, long, default_value_t=100)]
    kilos: u64
}

fn main() {
    let args = Arguments::parse();
    pretty_env_logger::init();

    info!("Arguments: {:?}", args);

    let mut data: LinkedList<i64> = LinkedList::from([69]);
    let segmentos: u64 = (100 + (args.kilos * 1000) + ((args.megas * 1000000.0) as u64) + ((args.gigas * 1000000000.0) as u64))/32;

    info!("Segments to Fill: {}", segmentos);

    for _ in 0..segmentos {
        data.push_back(rand::random());
    }

    info!("Bytes Filled: {}", segmentos*32);
    info!("Press any key to free the memory");
    io::stdin().read(&mut [0u8]).unwrap();
}
