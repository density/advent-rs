use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;

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
    V: Clone,
{
    pub fn new(default: V) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::default_map::DefaultHashMap;
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        let mut default_map: DefaultHashMap<String, usize> = DefaultHashMap::new(0);

        let key = "hello";

        assert!(!default_map.contains_key(key));
        assert_eq!(*default_map.get(key), 0);
        assert_eq!(default_map[key], 0);

        assert_eq!(default_map.insert(key.to_string(), 5), None);
        assert!(default_map.contains_key(key));
        assert_eq!(*default_map.get(key), 5);
        assert_eq!(default_map[key], 5);

        assert_eq!(*default_map.get_mut(key.to_string()), 5);

        *default_map.get_mut(key.to_string()) = 10;
        assert_eq!(default_map[key], 10);
    }

    #[test]
    fn test_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("hello".to_string(), 5_usize);

        let default_map: DefaultHashMap<String, usize> = map.into();

        assert_eq!(default_map["hello"], 5);
        assert_eq!(default_map["world"], 0);
    }
}
