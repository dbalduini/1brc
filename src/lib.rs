use measurement::Measurement;
use perfect_hash::PerfectHash;

pub mod measurement;
pub mod perfect_hash;

const NUMBER_OF_STATIONS: usize = 413;

pub fn aggregate(content: &str) -> PerfectHash {
    let mut map = PerfectHash::new();

    for line in content.lines() {
        if let Some(i) = line.find(";") {
            let (station, t) = line.split_at(i);
            let t = t[1..].parse::<f64>().unwrap();
            map.upsert(station, t);
        }
    }

    map
}
