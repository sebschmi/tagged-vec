use std::{fmt::Debug, marker::PhantomData};

use crate::TaggedVec;

impl<Index, Value> Extend<Value> for TaggedVec<Index, Value> {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.vec.extend(iter);
    }
}

impl<Index, Value> FromIterator<Value> for TaggedVec<Index, Value> {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self {
            index_type: PhantomData,
            vec: FromIterator::from_iter(iter),
        }
    }
}

impl<Index, Value> IntoIterator for TaggedVec<Index, Value> {
    type Item = Value;

    type IntoIter = <Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, Index, Value> IntoIterator for &'a TaggedVec<Index, Value> {
    type Item = &'a Value;

    type IntoIter = <&'a Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<'a, Index, Value> IntoIterator for &'a mut TaggedVec<Index, Value> {
    type Item = &'a mut Value;

    type IntoIter = <&'a mut Vec<Value> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter_mut()
    }
}

impl<Index, Value> From<Vec<Value>> for TaggedVec<Index, Value> {
    fn from(value: Vec<Value>) -> Self {
        Self {
            index_type: PhantomData,
            vec: value,
        }
    }
}

impl<Index, Value> From<TaggedVec<Index, Value>> for Vec<Value> {
    fn from(value: TaggedVec<Index, Value>) -> Self {
        value.vec
    }
}

impl<Index, Value: Debug> Debug for TaggedVec<Index, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TaggedVec{:?}", self.vec)
    }
}

impl<Index, Value: Clone> Clone for TaggedVec<Index, Value> {
    fn clone(&self) -> Self {
        Self {
            index_type: PhantomData,
            vec: self.vec.clone(),
        }
    }
}

impl<Index, Value> Default for TaggedVec<Index, Value> {
    fn default() -> Self {
        Self {
            index_type: PhantomData,
            vec: Default::default(),
        }
    }
}

impl<Index, Value: PartialEq> PartialEq for TaggedVec<Index, Value> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}

impl<Index, Value: Eq> Eq for TaggedVec<Index, Value> {}

impl<Index, Value: PartialOrd> PartialOrd for TaggedVec<Index, Value> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.vec.partial_cmp(&other.vec)
    }
}

impl<Index, Value: Ord> Ord for TaggedVec<Index, Value> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.vec.cmp(&other.vec)
    }
}

/////////////////////////////////////////
////// INDEXING /////////////////////////
/////////////////////////////////////////

impl<Index: Into<usize>, Value> std::ops::Index<Index> for TaggedVec<Index, Value> {
    type Output = Value;

    fn index(&self, index: Index) -> &Self::Output {
        &self.vec[index.into()]
    }
}

impl<Index: Into<usize>, Value> std::ops::IndexMut<Index> for TaggedVec<Index, Value> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.vec[index.into()]
    }
}
