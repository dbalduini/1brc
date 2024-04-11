use hashmap::StationsMap;

use std::fs;

pub mod hashmap;
pub mod measurement;

pub fn parse_file(path: &str) -> StationsMap {
    let mut map = StationsMap::new();

    let content = fs::read_to_string(path).unwrap();

    for line in content.lines() {
        process_line(line, &mut map);
    }

    map
}

fn process_line(line: &str, map: &mut StationsMap) {
    if let Some((station, t)) = line.split_once(";") {
        let t = t.parse::<f64>().unwrap();
        map.upsert(station, t);
    }
}
