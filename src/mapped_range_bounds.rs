use std::ops::{Bound, RangeBounds};

pub struct MappedRangeBounds {
    start_bound: Bound<usize>,
    end_bound: Bound<usize>,
}

impl MappedRangeBounds {
    pub fn new<Index>(range: impl RangeBounds<Index>) -> Self
    where
        usize: From<Index>,
        Index: Copy,
    {
        let start_bound = match range.start_bound() {
            Bound::Included(index) => Bound::Included((*index).into()),
            Bound::Excluded(index) => Bound::Excluded((*index).into()),
            Bound::Unbounded => Bound::Unbounded,
        };

        let end_bound = match range.end_bound() {
            Bound::Included(index) => Bound::Included((*index).into()),
            Bound::Excluded(index) => Bound::Excluded((*index).into()),
            Bound::Unbounded => Bound::Unbounded,
        };

        Self {
            start_bound,
            end_bound,
        }
    }
}

impl RangeBounds<usize> for MappedRangeBounds {
    fn start_bound(&self) -> Bound<&usize> {
        self.start_bound.as_ref()
    }

    fn end_bound(&self) -> Bound<&usize> {
        self.end_bound.as_ref()
    }
}
