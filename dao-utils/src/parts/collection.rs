use std::{fmt::Debug, hash::Hash, ops::Range};

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped,
};

use super::mapping::IndexedMapping;
use crate::{consts, Error, Instanced, Variable};

/// Data structure for storing indexed values.
///
/// It's is a wrapper on top of:
/// [`Variable`] - stores the current collection length.
/// [`IndexedMapping`] - stores index-value pairs.
pub struct OrderedCollection<T> {
    pub values: IndexedMapping<T>,
    pub length: Variable<u32>,
}

impl<T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash> OrderedCollection<T> {
    /// Creates a new OrderedCollection instance.
    pub fn new(name: &str) -> Self {
        Self {
            values: IndexedMapping::new(name.to_string()),
            length: Variable::new(format!("{}{}", name, consts::LENGTH_SUFFIX)),
        }
    }

    /// Tries to delete the given `item`. If succeeds, returns true, otherwise, returns false.
    ///
    /// Reindexes collection after successful removal.
    pub fn delete(&mut self, item: T) -> bool {
        let length = self.size();
        let (is_deleted, item_index) = self.values.remove(item);

        if !is_deleted {
            return false;
        }

        self.length.set(length - 1);
        let last_index = length - 1;
        // if the last item was removed, we are done here, no need to reindex
        if item_index == last_index {
            return true;
        }

        self.move_item(last_index, item_index);
        true
    }

    /// Gets the value under the given index. Returns `None` if the index does not exist.
    pub fn get(&self, index: u32) -> Option<T> {
        self.values.get(index)
    }

    fn move_item(&mut self, from: u32, to: u32) {
        let value = self.values.get(from);
        self.values
            .set(to, value.unwrap_or_revert_with(Error::StorageError));
        self.values.unset(from);
    }

    fn _add(&mut self, item: T) {
        let length = self.size();
        self.values.set(length, item);
        self.length.set(length + 1);
    }

    /// Returns an iterator.
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T> OrderedCollection<T> {
    /// Returns the collection size.
    pub fn size(&self) -> u32 {
        self.length.get().unwrap_or(0)
    }
}

/// A collection acts like a set.
pub trait Set<T> {
    fn add(&mut self, item: T);
}

impl<T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash> Set<T> for OrderedCollection<T> {
    fn add(&mut self, item: T) {
        if !self.values.contains(&item) {
            self._add(item);
        }
    }
}

/// A collection acts like a list.
pub trait List<T> {
    fn add(&mut self, item: T);
}

impl<T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash> List<T> for OrderedCollection<T> {
    fn add(&mut self, item: T) {
        self._add(item);
    }
}

impl<T: FromBytes + ToBytes + CLTyped> Instanced for OrderedCollection<T> {
    fn instance(namespace: &str) -> Self {
        Self {
            values: Instanced::instance(&format!("{}:{}", namespace, "values")),
            length: Instanced::instance(&format!("{}:{}", namespace, "length")),
        }
    }
}

/// [`OrderedCollection`] iterator.
pub struct Iter<'a, T> {
    collection: &'a OrderedCollection<T>,
    range: Range<u32>,
}

impl<'a, T> Iter<'a, T> {
    /// Returns a new instance of Iter.
    fn new(collection: &'a OrderedCollection<T>) -> Self {
        Self {
            collection,
            range: Range {
                start: 0,
                end: collection.size(),
            },
        }
    }

    /// Returns number of elements left to iterate.
    fn remaining(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }
}

impl<'a, T> core::iter::Iterator for Iter<'a, T>
where
    T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        <Self as Iterator>::nth(self, 0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.remaining()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let index = self.range.nth(n)?;
        self.collection.get(index)
    }
}

impl<'a, T> core::iter::ExactSizeIterator for Iter<'a, T> where
    T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash
{
}

impl<'a, T> core::iter::DoubleEndedIterator for Iter<'a, T>
where
    T: ToBytes + FromBytes + CLTyped + PartialEq + Debug + Hash,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.nth_back(0)?;
        self.collection.get(index)
    }
}
