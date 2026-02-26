/// An iterator over the indices of a `TaggedVec`.
pub struct IndexIterator<Index> {
    start_inclusive: usize,
    end_exclusive: usize,
    marker: std::marker::PhantomData<Index>,
}

impl<Index> IndexIterator<Index> {
    /// Creates a new `IndexIterator` for the given range.
    pub fn new(start_inclusive: usize, end_exclusive: usize) -> Self {
        Self {
            start_inclusive,
            end_exclusive,
            marker: std::marker::PhantomData,
        }
    }
}

impl<Index> Iterator for IndexIterator<Index>
where
    Index: From<usize>,
{
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_inclusive < self.end_exclusive {
            let index = self.start_inclusive;
            self.start_inclusive += 1;
            Some(Index::from(index))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end_exclusive - self.start_inclusive;
        (len, Some(len))
    }
}

impl<Index> DoubleEndedIterator for IndexIterator<Index>
where
    Index: From<usize>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start_inclusive < self.end_exclusive {
            self.end_exclusive -= 1;
            Some(Index::from(self.end_exclusive))
        } else {
            None
        }
    }
}

impl<Index> ExactSizeIterator for IndexIterator<Index> where Index: From<usize> {}
