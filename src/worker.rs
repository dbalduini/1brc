use crate::StationsMap;

use std::io::BufRead;
use std::thread;
use std::{fs, sync::Arc};

use memmap::Mmap;

pub struct Worker {
    id: usize,
    start: usize,
    end: usize,
    mmap: Arc<Mmap>,
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

    pub fn divide_work(&mut self, path: &str) {
        let file = fs::File::open(path).unwrap();
        let size: usize = file.metadata().unwrap().len().try_into().unwrap();

        let mmap = unsafe { Mmap::map(&file).unwrap() };

        println!("File size {}", size);

        let chunk_size = size / self.size;

        let mut start = 0;
        let mut chunks = Vec::new();

        for id in 0..self.size {
            // align the chunk
            let mut end = usize::min(start + chunk_size, size);
            while end < size && mmap[end] != b'\n' {
                end += 1;
            }
            chunks.push((id, start, end));
            start = end;
        }

        let mmap = Arc::new(mmap);

        for (id, start, end) in chunks {
            let w = Worker::new(id, start, end, Arc::clone(&mmap));
            self.workers.push(w);
        }
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
    pub fn new(id: usize, start: usize, end: usize, mmap: Arc<Mmap>) -> Self {
        Self {
            id,
            start,
            end,
            mmap,
        }
    }

    pub fn run(self) -> thread::JoinHandle<StationsMap> {
        thread::spawn(move || {
            // dbg!(self.id, self.start, self.end);

            // each thread has its own file descriptor
            let chunk = &self.mmap[self.start..self.end];

            let mut map = StationsMap::new();

            for line in chunk.lines() {
                process_line(line.unwrap(), &mut map);
            }

            map
        })
    }
}

#[inline]
fn process_line(line: String, map: &mut StationsMap) {
    // TODO: improve split_once
    if let Some((station, t)) = line.split_once(";") {
        // TODO: improve float parsing
        let t = t.parse::<f64>().unwrap();
        map.upsert_float(station, t);
    }
}
