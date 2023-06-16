use std::{collections::LinkedList, io::{self, Read}, time::Instant, sync::{Arc, Mutex}};

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

async fn fill_segments(segments: u64, list_lock: Arc<Mutex<LinkedList<i64>>>) {
    for _ in 0..segments {
        let mut list = list_lock.lock().unwrap();
        list.push_back(rand::random());
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    pretty_env_logger::init();

    info!("Arguments: {:?}", args);

    let mut data: LinkedList<Arc<Mutex<LinkedList<i64>>>> = LinkedList::new();
    let mut handles = vec![];
    let mut segmentos: u64 = (100 + (args.kilos * 1000) + ((args.megas * 1000000.0) as u64) + ((args.gigas * 1000000000.0) as u64))/32;

    info!("Segments to Fill: {}", segmentos);

    let now = Instant::now();
    info!("Starting fill at {}", chrono::Local::now());

    while segmentos>0 {
        let new_list= Arc::new(Mutex::new(LinkedList::<i64>::new()));
        data.push_back(Arc::clone(&new_list));

        let task_segments = if segmentos > 500000 {500000} else {segmentos};

        segmentos -= task_segments;

        handles.push(
            tokio::spawn(async move {
                fill_segments(task_segments, new_list).await
            })
        );
    }

    info!("Tasks started after {} ms, at {}", now.elapsed().as_millis(), chrono::Local::now());

    for task in handles {
        task.await.unwrap();
    }

    info!("Finished after {} ms, at {}", now.elapsed().as_millis(), chrono::Local::now());

    info!("Bytes Filled: {}", segmentos*32);
    info!("Press any key to free the memory");
    io::stdin().read(&mut [0u8]).unwrap();
}
