//! An alternative to the standard libraries' [`Vec`] which is indexed with a custom type instead of [`usize`].
//!
//! This is useful to catch errors like using the wrong variable to index the vector.

#![warn(missing_docs)]

use std::marker::PhantomData;

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

    /// Inserts the given value at the back of the `Vec`, returning its index.
    pub fn push(&mut self, value: Value) -> Index
    where
        Index: From<usize>,
    {
        let index = self.vec.len().into();
        self.vec.push(value);
        index
    }

    /// Removes the value at the back of the `Vec` and returns it with its index.
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
}
