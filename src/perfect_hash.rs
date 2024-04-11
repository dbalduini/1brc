use crate::measurement::Measurement;

const NUMBER_OF_STATIONS: usize = 413;

#[derive(Debug)]
pub struct Entry(String, Measurement);

pub struct PerfectHash {
    buckets: Vec<Option<Entry>>,
}

impl PerfectHash {
    pub fn new() -> Self {
        const NONE: Option<Entry> = None;
        let buckets = Vec::from([NONE; NUMBER_OF_STATIONS]);
        PerfectHash { buckets }
    }

    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 5381;
        for c in key.chars() {
            hash = (hash << 5).wrapping_add(hash) + (c as usize);
        }
        hash
    }

    pub fn upsert(&mut self, key: &str, value: f64) -> bool {
        let mut i = self.hash(&key) % NUMBER_OF_STATIONS;

        // Safety: we do a mod operation over the capacity.
        let mut slot = unsafe { self.buckets.get_unchecked_mut(i) };

        // check if slot is occupied
        while let Some(station) = slot {
            if station.0 == key {
                break;
            }
            // linear prob
            i = (i + 1) % NUMBER_OF_STATIONS;
            slot = unsafe { self.buckets.get_unchecked_mut(i) };
        }

        match slot.as_mut() {
            Some(entry) => {
                entry.1.update(value);
                false
            }
            None => {
                *slot = Some(Entry(key.to_owned(), Measurement::new(value)));
                true
            }
        }
    }

    pub fn entries(self) -> Vec<Option<Entry>> {
        self.buckets
    }
}
