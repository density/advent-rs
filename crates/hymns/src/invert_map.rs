use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use std::hash::{BuildHasher, Hash};

pub trait Invertible<K, V>
where
    Self: Sized,
{
    fn into_inverted(self) -> HashMap<V, K>;
    fn inverted(&self) -> HashMap<&V, &K>;

    fn into_inverted_with_dupes(self) -> HashMap<V, Vec<K>>;
    fn inverted_with_dupes(&self) -> HashMap<&V, Vec<&K>>;
}

impl<K, V, S: BuildHasher> Invertible<K, V> for HashMap<K, V, S>
where
    Self: Sized,
    K: Hash + Eq,
    V: Hash + Eq,
{
    fn into_inverted(self) -> HashMap<V, K> {
        let mut inverted = HashMap::new();

        for (k, v) in self {
            match inverted.entry(v) {
                Entry::Occupied(_) => {
                    panic!("Attempted to invert HashMap with duplicate value for key");
                }
                Entry::Vacant(entry) => {
                    entry.insert(k);
                }
            }
        }

        inverted
    }

    fn inverted(&self) -> HashMap<&V, &K> {
        let mut inverted = HashMap::new();

        for (k, v) in self {
            match inverted.entry(v) {
                Entry::Occupied(_) => {
                    panic!("Attempted to invert HashMap with duplicate value for key");
                }
                Entry::Vacant(entry) => {
                    entry.insert(k);
                }
            }
        }

        inverted
    }

    fn into_inverted_with_dupes(self) -> HashMap<V, Vec<K>> {
        let mut inverted = HashMap::new();

        for (k, v) in self {
            inverted.entry(v).or_insert_with(Vec::new).push(k);
        }

        inverted
    }

    fn inverted_with_dupes(&self) -> HashMap<&V, Vec<&K>> {
        let mut inverted = HashMap::new();

        for (k, v) in self {
            inverted.entry(v).or_insert_with(Vec::new).push(k);
        }

        inverted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_into_inverted_no_dupes() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");

        let mut inverted = HashMap::new();
        inverted.insert("hello", 1);
        inverted.insert("world", 3);

        assert_eq!(map.into_inverted(), inverted);
    }

    #[test]
    fn test_inverted_no_dupes() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");

        let inverted = map.inverted();

        assert_eq!(inverted.len(), 2);
        assert_eq!(inverted[&"hello"], &1);
        assert_eq!(inverted[&"world"], &3);
    }

    #[test]
    #[should_panic(expected = "Attempted to invert HashMap with duplicate value for key")]
    fn test_into_inverted_with_dupes_not_allowed() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");
        map.insert(4, "hello");

        map.into_inverted();
    }

    #[test]
    #[should_panic(expected = "Attempted to invert HashMap with duplicate value for key")]
    fn test_inverted_with_dupes_not_allowed() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");
        map.insert(4, "hello");

        map.inverted();
    }

    #[test]
    fn test_into_inverted_with_dupes() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");
        map.insert(4, "hello");

        let inverted = map.into_inverted_with_dupes();

        assert_eq!(inverted.len(), 2);
        assert_eq!(inverted["world"], vec![3]);
        assert_eq!(
            inverted["hello"].iter().sorted().collect_vec(),
            vec![&1, &4]
        );
    }

    #[test]
    fn test_inverted_with_dupes() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        map.insert(3, "world");
        map.insert(4, "hello");

        let inverted = map.inverted_with_dupes();

        assert_eq!(inverted.len(), 2);
        assert_eq!(inverted[&"world"], vec![&3]);
        assert_eq!(
            inverted[&"hello"].iter().sorted().collect_vec(),
            vec![&&1, &&4]
        );
    }
}
