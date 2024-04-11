use crate::measurement::Measurement;

const NUMBER_OF_STATIONS: usize = 413;

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

    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 5381;
        for c in key.chars() {
            hash = (hash << 5).wrapping_add(hash) + (c as usize);
        }
        hash
    }

    pub fn upsert(&mut self, key: &str, value: f64) -> () {
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
            Some(entry) => entry.1.update(value),
            None => *slot = Some(Entry(key.to_owned(), Measurement::new(value))),
        }
    }

    pub fn entries(self) -> Vec<Option<Entry>> {
        let mut buckets = self.buckets;
        buckets.sort_unstable_by(|a, b| a.as_ref().unwrap().0.cmp(&b.as_ref().unwrap().0));
        buckets
    }
}
