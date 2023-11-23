//! Concurrent vector sum implementation in Rust Rayon library.
//!

use rayon::prelude::*;
use std::ops::Add;

/// concurrent_sum - calculate vector sums concurrently
/// using Rust rayon library
pub fn concurrent_sum_rayon<T>(numbers: Vec<T>, num_threads: usize) -> T
where
    T: Add<Output = T> + Copy + Default + Send + Sync + 'static,
{
    if num_threads == 0 {
        panic!("The number of threads should be more than zero");
    }

    if num_threads > numbers.len() {
        panic!("The number of threads cannot be more than the number of elements");
    }

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    numbers
        .par_iter()
        .cloned()
        .reduce(|| T::default(), |a, b| a + b)
}
