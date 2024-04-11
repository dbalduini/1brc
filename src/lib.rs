use hashmap::StationsMap;

pub mod measurement;
pub mod hashmap;


pub fn aggregate(content: &str) -> StationsMap {
    let mut map = StationsMap::new();

    for line in content.lines() {
        if let Some(i) = line.find(";") {
            let (station, t) = line.split_at(i);
            let t = t[1..].parse::<f64>().unwrap();
            map.upsert(station, t);
        }
    }

    map
}
