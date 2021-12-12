use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug, Clone)]
pub struct Counter<T: Eq + Hash + Clone> {
    counts: HashMap<T, usize>,
}

impl<T: Clone + Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &T) {
        *self.counts.entry(key.clone()).or_insert(0) += 1
    }

    pub fn increment_count(&mut self, key: &T, count: usize) {
        *self.counts.entry(key.clone()).or_insert(0) += count
    }

    pub fn contains(&mut self, key: &T) -> bool {
        self.counts.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.counts.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn into_keys(self) -> impl Iterator<Item = T> {
        self.counts.into_keys()
    }

    pub fn counts(&self) -> impl Iterator<Item = &usize> {
        self.counts.values()
    }

    pub fn into_counts(self) -> impl Iterator<Item = usize> {
        self.counts.into_values()
    }
}

impl<T: Clone + Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterator: I) -> Self {
        let mut counter = Self::new();

        for item in iterator {
            counter.add(&item);
        }

        counter
    }
}

impl<T: Clone + Eq + Hash> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}
