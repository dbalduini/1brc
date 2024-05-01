use crate::StationsMap;

use std::io::{BufRead, Read};
use std::thread;
use std::{fs, sync::Arc};

use memmap::{Mmap, MmapOptions};

pub struct Worker {
    id: usize,
    chunk_size: usize,
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

        let mmap = unsafe { Arc::new(MmapOptions::new().map(&file).unwrap()) };

        println!("File size {}", size);

        let chunk_size = size / self.size;

        for id in 0..self.size {
            let w = Worker::new(id, chunk_size, Arc::clone(&mmap));
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
    pub fn new(id: usize, chunk_size: usize, mmap: Arc<Mmap>) -> Self {
        Self {
            id,
            chunk_size,
            mmap,
        }
    }

    pub fn run(self) -> thread::JoinHandle<StationsMap> {
        thread::spawn(move || {
            // each thread has its own file descriptor
            let chunk_size = self.chunk_size;
            let offset = self.id * self.chunk_size;

            let mut chunk = &self.mmap[offset..offset + chunk_size];

            let mut map = StationsMap::new();
            let mut c = 0;
            let mut buf = Vec::with_capacity(1024);

            while let Ok(n) = chunk.read_until(b'\n', &mut buf) {
                if n == 0 {
                    break;
                }

                let line = &buf[..n - 1];

                process_line(line, &mut map);

                c += 1;
                buf.clear();
            }

            dbg!(c);
            map
        })
    }
}

#[inline]
fn process_line(line: &[u8], map: &mut StationsMap) {
    let mut offset = 0;
    for c in line {
        if *c != b';' {
            offset += 1;
        }
    }
    // dbg!(String::from_utf8_lossy(&line[..offset]));
}
