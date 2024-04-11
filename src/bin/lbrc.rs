use lbrc::aggregate;
use std::{env, fs};

fn main() {
    // <string: station name>;<double: measurement>
    let file = env::args().skip(1).next().unwrap();

    println!("Reading file: {file}");
    let content = fs::read_to_string(file).unwrap();

    // let s = &content[0..1024];
    // println!("{s}");

    let map = aggregate(&content);

    // let mut xs: Vec<&str> = map.keys().map(|x| x.to_owned()).into_iter().collect();
    // xs.sort_unstable_by(|a, b| a.cmp(b));

    for entry in map.entries().iter().flatten() {
        println!("{} {}", entry.0, entry.1);
    }
}
