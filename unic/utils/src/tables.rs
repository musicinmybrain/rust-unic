//! bananas

use std::cmp;

/// bananas
pub trait CharBsearchTable<V> {
    /// bananas
    fn find(&self, needle: char) -> Option<V>;

    /// bananas
    fn binary_search_or(&self, needle: char, default: V) -> V {
        self.find(needle).unwrap_or(default)
    }

    /// bananas
    fn binary_search_or_else(&self, needle: char, f: fn() -> V) -> V {
        self.find(needle).unwrap_or_else(f)
    }
}

impl<'a, V> CharBsearchTable<&'a V> for &'a [(char, V)] {
    fn find(&self, needle: char) -> Option<&'a V> {
        self.binary_search_by_key(&needle, |&(k, _)| k)
            .map(|idx| &self[idx].1)
            .ok()
    }
}

impl<'a, V> CharBsearchTable<&'a V> for &'a [(char, char, V)] {
    fn find(&self, needle: char) -> Option<&'a V> {
        self.binary_search_by(|&(low, high, _)| if low > needle {
            cmp::Ordering::Greater
        } else if high < needle {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }).map(|idx| &self[idx].2)
            .ok()
    }
}

impl CharBsearchTable<()> for &'static [(char, char)] {
    fn find(&self, needle: char) -> Option<()> {
        self.binary_search_by(|&(low, high)| if low > needle {
            cmp::Ordering::Greater
        } else if high < needle {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }).map(|_| ())
            .ok()
    }
}
