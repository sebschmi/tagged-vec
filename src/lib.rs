//! An alternative to the standard libraries' [`Vec`] which is indexed with a custom type instead of [`usize`].
//!
//! This is useful to catch errors like using the wrong variable to index the vector.

#![warn(missing_docs)]

use std::{marker::PhantomData, ops::RangeBounds};

use mapped_range_bounds::MappedRangeBounds;

mod mapped_range_bounds;
mod trait_impls;

/// A [`Vec`] wrapper that allows indexing only via the given `Index` type.
///
/// For actual operation, `Index` must implement [`From<usize>`] and [`Into<usize>`].
pub struct TaggedVec<Index, Value> {
    index_type: PhantomData<Index>,
    vec: Vec<Value>,
}

impl<Index, Value> TaggedVec<Index, Value> {
    /// Creates a new empty `TaggedVec`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements in the `TaggedVec`.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns `true` if the `TaggedVec` contains no elements.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Inserts the given value at the back of the `TaggedVec`, returning its index.
    pub fn push(&mut self, value: Value) -> Index
    where
        Index: From<usize>,
    {
        let index = self.vec.len().into();
        self.vec.push(value);
        index
    }

    /// Removes the value at the back of the `TaggedVec` and returns it with its index.
    pub fn pop(&mut self) -> Option<(Index, Value)>
    where
        Index: From<usize>,
    {
        if let Some(value) = self.vec.pop() {
            Some((self.vec.len().into(), value))
        } else {
            None
        }
    }

    /// Inserts the given `value` at position `index`, shifting all existing values in range `index..` one position to the right.
    pub fn insert(&mut self, index: Index, value: Value)
    where
        Index: Into<usize>,
    {
        self.vec.insert(index.into(), value);
    }

    /// See [`Vec::splice`].
    pub fn splice<I: IntoIterator<Item = Value>>(
        &mut self,
        range: impl RangeBounds<Index>,
        replace_with: I,
    ) -> std::vec::Splice<'_, I::IntoIter>
    where
        usize: for<'a> From<&'a Index>,
    {
        self.vec.splice(MappedRangeBounds::new(range), replace_with)
    }

    /// Retains only the values specified by the predicate.
    ///
    /// In other words, remove all values `v` for which `f(&v)` returns `false`.
    /// This method operates in place, visiting each value exactly once in the original order, and preserves the order of the retained values.
    pub fn retain(&mut self, f: impl FnMut(&Value) -> bool) {
        self.vec.retain(f);
    }

    /// Returns an iterator over references to the entries of the `TaggedVec`.
    pub fn iter(&self) -> impl Iterator<Item = (Index, &Value)>
    where
        Index: From<usize>,
    {
        self.vec
            .iter()
            .enumerate()
            .map(|(index, value)| (index.into(), value))
    }

    /// Returns an iterator over mutable references to the entries of the `TaggedVec`.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Index, &mut Value)>
    where
        Index: From<usize>,
    {
        self.vec
            .iter_mut()
            .enumerate()
            .map(|(index, value)| (index.into(), value))
    }

    /// Returns an iterator over references to the values of the `TaggedVec`.
    pub fn iter_values(&self) -> std::slice::Iter<'_, Value> {
        self.vec.iter()
    }

    /// Returns an iterator over mutable references to the values of the `TaggedVec`.
    pub fn iter_values_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.vec.iter_mut()
    }

    /// Returns an iterator over the indices of the `TaggedVec`.
    pub fn iter_indices(&self) -> impl Iterator<Item = Index>
    where
        Index: From<usize>,
    {
        (0..self.vec.len()).map(Into::into)
    }
}
