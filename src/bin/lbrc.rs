use std::env;

use lbrc::parse_file;

fn main() {
    // <string: station name>;<double: measurement>
    let path = env::args().skip(1).next().unwrap();

    println!("Reading file: {path}");

    let map = parse_file(&path);

    for entry in map.entries() {
        println!("{} {}", entry.0, entry.1);
    }
}
