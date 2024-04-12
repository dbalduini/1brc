use hashmap::StationsMap;
use worker::WorkerPool;

use std::{fs, io::BufReader};

pub mod hashmap;
pub mod measurement;
pub mod worker;


pub fn aggregate_stations(path: &str) -> StationsMap {
    // let content = fs::read_to_string(path).unwrap();


    // returns the number of logical processors (hyperthreading)
    let num_workers = std::thread::available_parallelism().unwrap().get() - 1;
    // let num_workers = 4;
    
    println!("using {} workers", num_workers);
    let mut pool = WorkerPool::new(num_workers);

    pool.divide_work(path);
    let map = pool.run();

    map
}
