use crate::StationsMap;

use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

pub struct Worker {
    start: usize,
    end: usize,
    content: Arc<String>,
}

pub struct WorkerPool {
    size: usize,
    workers: Vec<Worker>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            workers: Vec::with_capacity(size),
        }
    }

    pub fn divide_work(&mut self, content: String) {
        let chunk_size = content.len() / self.size;
        let mut offset = 0;
        let mut chunk_end = chunk_size;

        let content = Arc::new(content);

        while chunk_end <= content.len() {
            // increase chunk size until next \n char
            while content.as_bytes()[chunk_end] != b'\n' {
                chunk_end += 1;
            }

            self.workers.push(Worker::new(offset, chunk_end, Arc::clone(&content)));

            offset = chunk_end + 1;
            chunk_end = chunk_end + chunk_size;
        }

        // last worker will process the remainder
        self.workers.push(Worker::new(offset, content.len() - 1, Arc::clone(&content)));
    }

    pub fn run(self) -> StationsMap {
        let mut runners = Vec::new();

        // Run jobs in parallel
        for worker in self.workers {
            runners.push(worker.run());
        }

        // Merge Results
        let mut map = StationsMap::new();
        for runner in runners {
            let res = runner.join().unwrap();
            map.merge(res);
        }
        map
    }
}

impl Worker {
    pub fn new(start: usize, end: usize, content: Arc<String>) -> Self {
        Self {
            start,
            end,
            content,
        }
    }

    pub fn run(self) -> JoinHandle<StationsMap> {
        thread::spawn(move || {
            let chunk = &self.content[self.start..self.end];
            let mut map = StationsMap::new();
            for line in chunk.lines() {
                process_line(line, &mut map)
            }
            map
        })
    }
}

fn process_line(line: &str, map: &mut StationsMap) {
    if let Some((station, t)) = line.split_once(";") {
        let t = t.parse::<f64>().unwrap();
        map.upsert_float(station, t);
    }
}
