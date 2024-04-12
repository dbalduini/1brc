use std::env;

use lbrc::aggregate_stations;

fn main() {
    // <string: station name>;<double: measurement>
    let path = env::args().skip(1).next().unwrap();

    println!("Reading file: {path}");

    let map = aggregate_stations(&path);

    for entry in map.entries() {
        println!("{} {}", entry.0, entry.1);
    }
}
