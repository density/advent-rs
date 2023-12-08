use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

#[derive(Default, Debug, Clone)]
pub struct Counter<T: Eq + Hash> {
    counts: HashMap<T, usize>,
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: T) {
        *self.counts.entry(key).or_insert(0) += 1
    }

    pub fn increment_count(&mut self, key: T, count: usize) {
        *self.counts.entry(key).or_insert(0) += count
    }

    pub fn remove(&mut self, key: &T) -> Option<usize> {
        self.counts.remove(key)
    }

    pub fn contains(&mut self, key: &T) -> bool {
        self.counts.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
        self.counts.iter().map(|(k, v)| (k, *v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn into_keys(self) -> impl Iterator<Item = T> {
        self.counts.into_keys()
    }

    pub fn counts(&self) -> impl Iterator<Item = usize> + '_ {
        self.counts.values().cloned()
    }

    pub fn into_counts(self) -> impl Iterator<Item = usize> {
        self.counts.into_values()
    }

    pub fn get(&self, k: &T) -> Option<usize> {
        self.counts.get(k).cloned()
    }

    pub fn into_map(self) -> HashMap<T, usize> {
        self.counts
    }
}

// impl Index for counter
impl<T: Eq + Hash> Index<&T> for Counter<T> {
    type Output = usize;

    fn index(&self, index: &T) -> &Self::Output {
        &self.counts[index]
    }
}

impl<T: Clone + Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iterator: I) -> Self {
        let mut counter = Self::new();

        for item in iterator {
            counter.add(item);
        }

        counter
    }
}

impl<T: Clone + Eq + Hash> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}
