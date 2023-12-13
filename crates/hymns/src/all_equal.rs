use std::cmp::Ordering;

/// Helper struct for ignoring order when comparing structs. Useful when you want to sort
/// tuples but only based on certain elements.
#[derive(Debug, Hash, Default, Eq, PartialEq)]
pub struct AllEqual<T>(pub T);

impl<T: Eq> PartialOrd for AllEqual<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for AllEqual<T> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T: Clone> Clone for AllEqual<T> {
    fn clone(&self) -> Self {
        AllEqual(self.0.clone())
    }
}
