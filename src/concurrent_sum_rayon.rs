//! Concurrent vector sum implementation in Rust Rayon library.
//!

use std::ops::Add;
use rayon::prelude::*;

fn concurrent_sum_rayon<T>(numbers: Vec<T>, num_threads: usize) -> T
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

    numbers.par_iter().cloned().reduce(|| T::default(), |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_valid() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sequential: u64 = vec.iter().cloned().sum();
        assert_eq!(sequential, concurrent_sum_rayon(vec, 2));
    }

    #[test]
    #[should_panic]
    fn sum_panic_more_threads() {
        let vec = vec![1];
        _ = concurrent_sum_rayon(vec, 2);
    }

    #[test]
    #[should_panic]
    fn sum_panic_zero_threads() {
        let vec = vec![1];
        _ = concurrent_sum_rayon(vec, 0);
    }
}
