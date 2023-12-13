use std::hash::Hash;

use crate::default_map::DefaultHashMap;

pub type Counter<T> = DefaultHashMap<T, usize>;

pub trait MoreItertools: Iterator {
    fn collect_counter(self) -> DefaultHashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Hash + Eq,
    {
        self.fold(DefaultHashMap::new(), |mut counts, item| {
            *counts.get_mut(item) += 1;
            counts
        })
    }
}

impl<T> MoreItertools for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_collect_counter() {
        let v = vec![1, 2, 3, 3];
        let counter = v.into_iter().collect_counter();

        assert_eq!(
            counter.into_iter().sorted().collect_vec(),
            vec![(1, 1), (2, 1), (3, 2)]
        );
    }
}
