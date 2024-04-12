use crate::StationsMap;

use std::fs;
use std::io;
use std::io::{BufRead, Read, Seek};
use std::os::windows::fs::FileExt;
use std::thread;

pub struct Worker {
    path: String,
    id: usize,
    chunk_size: usize,
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
        // let mut reader = BufReader::new(file);

        println!("File size {}", size);

        let chunk_size = size / self.size;
        let buffer_size = chunk_size;

        for id in 0..self.size {
            let w = Worker::new(path.to_string(), id, chunk_size);
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
    pub fn new(path: String, id: usize, chunk_size: usize) -> Self {
        Self {
            path,
            id,
            chunk_size,
        }
    }

    pub fn run(self) -> thread::JoinHandle<StationsMap> {
        thread::spawn(move || {
            // each thread has its own file descriptor
            let mut file = fs::File::open(self.path).unwrap();

            let chunk_size: u64 = self.chunk_size.try_into().unwrap();
            let offset: u64 = (self.id * self.chunk_size).try_into().unwrap();

            // fix the chunk position
            let offset = cut_chunk(&file, offset).unwrap();
            let chunk_size = expand_chunk(&file, offset, chunk_size).unwrap();

            // set the file cursor to thread offset
            file.seek(io::SeekFrom::Start(offset)).unwrap();

            // limit the file to chunk size so we dont have access to the hole file
            let file = file.take(chunk_size);

            // Buffered reader because I cant fit 16GB (1B rows file) in memory
            // Using a 4MB sized buffer
            let reader = io::BufReader::with_capacity(1024 * 1024 * 4, file);

            let mut map = StationsMap::new();
            for line in reader.lines() {
                process_line(line.unwrap(), &mut map);
            }
            map
        })
    }
}

fn process_line(line: String, map: &mut StationsMap) {
    if let Some((station, t)) = line.split_once(";") {
        match t.parse::<f64>() {
            Ok(t) => map.upsert_float(station, t),
            _ => (), //println!("failed to parse floag: {}", line),
        }
    }
}

fn cut_chunk(file: &fs::File, offset: u64) -> Result<u64, io::Error> {
    let mut offset = offset;
    let mut buf: Vec<u8> = vec![0; 32];

    file.seek_read(&mut buf, offset)?;

    let chunk_head = String::from_utf8_lossy(&buf);

    for c in chunk_head.chars() {
        if c > 'A' && c < 'Z' {
            break;
        }
        offset += 1;
    }

    Ok(offset)
}

fn expand_chunk(file: &fs::File, offset: u64, chunk_size: u64) -> Result<u64, std::io::Error> {
    let mut chunk_size = chunk_size;
    let mut buf: Vec<u8> = vec![0; 32];

    let n = file.seek_read(&mut buf, offset + chunk_size)?;

    let chunk_tail = String::from_utf8_lossy(&buf[..n]);

    for c in chunk_tail.chars() {
        if c == '\n' {
            break;
        }
        chunk_size += 1;
    }

    Ok(chunk_size)
}
