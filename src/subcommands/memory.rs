use std::{sync::{Arc, Mutex}, collections::LinkedList, time::Instant, io::{stdin, Read}};

use log::{info, trace};

use crate::args::MemoryArgs;

async fn fill_segments(segments: u64, list_lock: Arc<Mutex<LinkedList<i64>>>) {
    for _ in 0..segments {
        let mut list = list_lock.lock().unwrap();
        list.push_back(rand::random());
    }

    trace!("{} segments done.", segments)
}

/// Executes Subcommand Fill Memory
pub async fn fill_memory(args: MemoryArgs) {
    let mut data: LinkedList<Arc<Mutex<LinkedList<i64>>>> = LinkedList::new();
    let mut handles = vec![];
    let total_memory = 100 + (args.kilos * 1000) + ((args.megas * 1000000.0) as u64) + ((args.gigas * 1000000000.0) as u64);
    let mut segmentos: u64 = total_memory/32;

    info!("Segments to Fill: {}", segmentos);

    let now = Instant::now();
    info!("Starting fill at {}", chrono::Local::now());

    while segmentos>0 {
        let new_list= Arc::new(Mutex::new(LinkedList::<i64>::new()));
        data.push_back(Arc::clone(&new_list));

        let segments_per_task: u64 = 2000000;
        let task_segments = if segmentos > segments_per_task {segments_per_task} else {segmentos};

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

    info!("Bytes Filled: {}", total_memory);
    info!("Press any key to free the memory");
    stdin().read(&mut [0u8]).unwrap();
}