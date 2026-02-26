use std::{
    iter,
    time::{Duration, Instant},
};

use crate::TaggedVec;

#[test]
fn delete_multi() {
    let mut v = TaggedVec::<usize, _>::from_iter([0, 1, 2, 3, 4]);
    v.remove_multi([0, 4]);
    assert_eq!(v, vec![1, 2, 3].into());

    let mut v = TaggedVec::<usize, _>::from_iter([0, 1, 2, 3, 4]);
    v.remove_multi([0, 2, 4]);
    assert_eq!(v, vec![1, 3].into());

    let mut v = TaggedVec::<usize, _>::from_iter([0, 1, 2, 3, 4]);
    v.remove_multi([1, 3]);
    assert_eq!(v, vec![0, 2, 4].into());
}

#[test]
fn iter_skip() {
    #[derive(Copy, Clone)]
    struct Index(usize);

    impl From<usize> for Index {
        fn from(value: usize) -> Self {
            Self(value)
        }
    }

    impl From<Index> for usize {
        fn from(value: Index) -> Self {
            value.0
        }
    }

    // 128 MiB of data.
    let v = TaggedVec::<Index, u8>::from_iter(iter::repeat_n(0..255, 4 * 1024 * 128).flatten());

    let start_time = Instant::now();
    let sum: usize = v
        .iter(..Index(10))
        .map(|(index, value)| index.0 + *value as usize)
        .sum();
    let no_skip_time = start_time.elapsed();

    let start_time = Instant::now();
    let sum_skip: usize = v
        .iter(Index(v.len() - 20)..Index(v.len() - 10))
        .map(|(index, value)| index.0 + *value as usize)
        .sum();
    let skip_time = start_time.elapsed();

    println!("Sum: {sum}");
    println!("Skip sum: {sum_skip}");
    println!("Time without skip: {no_skip_time:?}");
    println!("Time with skip: {skip_time:?}");

    assert!(
        (skip_time < no_skip_time * 2 && no_skip_time < skip_time * 2)
            || skip_time < Duration::from_millis(10) && no_skip_time < Duration::from_millis(10)
    );
}
