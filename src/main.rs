use std::{collections::HashMap, fs};

struct Measurement {
    min: f64,
    max: f64,
    total: f64,
    count: usize,
}

impl Measurement {
    fn new(t: f64) -> Self {
        Measurement {
            min: t,
            max: t,
            count: 1,
            total: t,
        }
    }

    fn update(&mut self, t: f64) -> () {
        self.min = f64::min(self.min, t);
        self.max = f64::max(self.max, t);
        self.count += 1;
        self.total += t;
    }
}

impl std::fmt::Display for Measurement {
    // <min>/<mean>/<max>
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.min,
            self.total / self.count as f64,
            self.max
        )
    }
}

fn main() {
    // <string: station name>;<double: measurement>
    let file = std::env::args().skip(1).next().unwrap();

    println!("Reading file: {file}");
    let content = fs::read_to_string(file).unwrap();

    // let s = &content[0..1024];
    // println!("{s}");

    let mut map: HashMap<&str, Measurement> = HashMap::with_capacity(2048);

    for line in content.lines() {
        if let Some(i) = line.find(";") {
            let (station, t) = line.split_at(i);
            let t = t[1..].parse::<f64>().unwrap();
            match map.get_mut(station) {
                Some(m) => {
                    m.update(t);
                }
                None => {
                    map.insert(station, Measurement::new(t));
                }
            }
        }
    }

    let mut xs = map.into_iter().collect::<Vec<(&str, Measurement)>>();
    xs.sort_unstable_by(|a, b| a.0.cmp(b.0));

    for (k, v) in &xs[..20] {
        println!("{}: {}", k, v);
    }
}
