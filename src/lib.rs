use hashmap::StationsMap;
use worker::WorkerPool;

use std::fs;

pub mod hashmap;
pub mod measurement;
pub mod worker;


pub fn parse_file(path: &str) -> StationsMap {
    let content = fs::read_to_string(path).unwrap();

    // returns the number of logical processors (hyperthreading)
    let num_workers = std::thread::available_parallelism().unwrap().get() - 1;
    
    dbg!(num_workers);
    let mut pool = WorkerPool::new(num_workers);

    pool.divide_work(content);
    let map = pool.run();

    map
}
