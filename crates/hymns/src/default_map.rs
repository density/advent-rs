use hashbrown::{hash_map, HashMap};
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct DefaultHashMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    map: HashMap<K, V>,
    default: V,
}

impl<K, V> DefaultHashMap<K, V>
where
    K: Hash + Eq,
    V: Clone + Default,
{
    #[must_use]
    pub fn new() -> Self {
        Self::with_default(V::default())
    }
}

impl<K, V> Default for DefaultHashMap<K, V>
where
    K: Hash + Eq,
    V: Clone + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> DefaultHashMap<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn with_default(default: V) -> Self {
        Self {
            map: HashMap::new(),
            default,
        }
    }

    pub fn get<Q>(&self, key: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(key.borrow()).unwrap_or(&self.default)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        self.map.entry(key).or_insert_with(|| self.default.clone())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    pub fn into_keys(self) -> impl Iterator<Item = K> {
        self.map.into_keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.map.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.map.values()
    }

    pub fn into_values(self) -> impl Iterator<Item = V> {
        self.map.into_values()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.contains_key(key)
    }
}

impl<K, Q, V> Index<&Q> for DefaultHashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    V: Clone,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key)
    }
}

impl<K, V> From<HashMap<K, V>> for DefaultHashMap<K, V>
where
    K: Eq + Hash,
    V: Default + Clone,
{
    fn from(map: HashMap<K, V>) -> Self {
        Self {
            map,
            default: Default::default(),
        }
    }
}

impl<K, V> IntoIterator for DefaultHashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::default_map::DefaultHashMap;
    use hashbrown::HashMap;
    use itertools::Itertools;

    #[test]
    fn test_hashmap() {
        let mut default_map: DefaultHashMap<String, usize> = DefaultHashMap::new();

        let key = "hello";

        assert!(!default_map.contains_key(key));
        assert_eq!(*default_map.get(key), 0);
        assert_eq!(default_map[key], 0);

        assert_eq!(default_map.insert(key.to_string(), 5), None);

        assert_eq!(default_map.remove(key), Some(5));

        assert_eq!(default_map.insert(key.to_string(), 5), None);
        assert_eq!(default_map.insert(key.to_string(), 5), Some(5));

        assert!(default_map.contains_key(key));
        assert_eq!(*default_map.get(key), 5);
        assert_eq!(default_map[key], 5);

        assert_eq!(*default_map.get_mut(key.to_string()), 5);

        *default_map.get_mut(key.to_string()) = 10;
        assert_eq!(default_map[key], 10);

        assert_eq!(default_map.keys().collect_vec(), vec!["hello"]);
        assert_eq!(default_map.values().collect_vec(), vec![&10]);

        assert_eq!(default_map.clone().into_keys().collect_vec(), vec!["hello"]);
        assert_eq!(default_map.clone().into_values().collect_vec(), vec![10]);
    }

    #[test]
    fn test_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), 5_usize);

        let default_map: DefaultHashMap<String, usize> = map.into();

        assert_eq!(default_map["hello"], 5);
        assert_eq!(default_map["world"], 0);
    }

    #[test]
    fn test_hashmap_with_explicit_default() {
        let default_map: DefaultHashMap<String, usize> = DefaultHashMap::with_default(5);
        assert_eq!(default_map["hello"], 5);
    }
}
