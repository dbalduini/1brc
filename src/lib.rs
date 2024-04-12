use hashmap::StationsMap;
use worker::WorkerPool;

use std::fs;

pub mod hashmap;
pub mod measurement;
pub mod worker;


pub fn parse_file(path: &str) -> StationsMap {
    let content = fs::read_to_string(path).unwrap();

    let num_workers = 4;
    let mut pool = WorkerPool::new(num_workers);

    pool.divide_work(content);
    let map = pool.run();

    map
}
