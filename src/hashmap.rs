use crate::measurement::Measurement;

// const NUMBER_OF_STATIONS: usize = 413;
const NUMBER_OF_STATIONS: usize = 1024;

#[derive(Debug)]
pub struct Entry(pub String, pub Measurement);

pub struct StationsMap {
    buckets: Vec<Option<Entry>>,
}

impl StationsMap {
    pub fn new() -> Self {
        const NONE: Option<Entry> = None;
        let buckets = Vec::from([NONE; NUMBER_OF_STATIONS]);
        Self { buckets }
    }

    #[inline]
    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 5381;
        for c in key.chars() {
            hash = (hash << 5).wrapping_add(hash) + (c as usize);
        }
        hash
    }

    pub fn upsert_float(&mut self, key: &str, value: f64) -> () {
        let mut i = self.hash(&key) % NUMBER_OF_STATIONS;

        // Safety: we do a mod operation over the capacity.
        let mut slot = unsafe { self.buckets.get_unchecked_mut(i) };

        // linear probe
        while let Some(station) = slot {
            if station.0 == key {
                break;
            }
            i = (i + 1) % NUMBER_OF_STATIONS;
            slot = unsafe { self.buckets.get_unchecked_mut(i) };
        }

        match slot.as_mut() {
            Some(entry) => entry.1.update_float(value),
            None => *slot = Some(Entry(key.to_owned(), Measurement::new(value))),
        }
    }

    pub fn upsert(&mut self, key: &str, measurement: Measurement) -> () {
        let mut i = self.hash(&key) % NUMBER_OF_STATIONS;
        // let mut i = measurement.hash;

        // Safety: we do a mod operation over the capacity.
        let mut slot = unsafe { self.buckets.get_unchecked_mut(i) };

        // linear probe
        while let Some(station) = slot {
            if station.0 == key {
                break;
            }
            i = (i + 1) % NUMBER_OF_STATIONS;
            slot = unsafe { self.buckets.get_unchecked_mut(i) };
            // dbg!(key);
        }

        match slot.as_mut() {
            Some(entry) => entry.1.update(&measurement),
            None => *slot = Some(Entry(key.to_owned(), measurement)),
        }
    }

    pub fn entries(self) -> Vec<Entry> {
        let mut buckets: Vec<Entry> = self.buckets.into_iter().flatten().collect();
        buckets.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        buckets
    }

    pub fn merge(&mut self, other: StationsMap) {
        for entry in other.buckets.into_iter() {
            if let Some(entry) = entry {
                self.upsert(&entry.0, entry.1);
            }
        }
    }
}
