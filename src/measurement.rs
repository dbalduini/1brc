use std::fmt;

pub struct Measurement {
    min: f64,
    max: f64,
    total: f64,
    count: usize,
}

impl Measurement {
    pub fn new(t: f64) -> Self {
        Measurement {
            min: t,
            max: t,
            count: 1,
            total: t,
        }
    }

    pub fn update(&mut self, t: f64) -> () {
        self.min = f64::min(self.min, t);
        self.max = f64::max(self.max, t);
        self.count += 1;
        self.total += t;
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}", // <min>/<mean>/<max>
            self.min,
            self.total / self.count as f64,
            self.max
        )
    }
}

impl fmt::Debug for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}", // <min>/<mean>/<max>
            self.min,
            self.total / self.count as f64,
            self.max
        )
    }
}