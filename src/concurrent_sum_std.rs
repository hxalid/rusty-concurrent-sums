//! Concurrent vector sum implementation in standard Rust.
//!

use std::cmp::min;
use std::ops::AddAssign;
use std::sync::Arc;
use std::thread::spawn;

/// concurrent_sum - calculate vector sums concurrently
/// using Rust standard library
pub fn concurrent_sum<T>(numbers: Vec<T>, num_threads: usize) -> T
where
    T: AddAssign + Copy + Default + Send + Sync + 'static,
{
    if num_threads == 0 {
        panic!("The number of threads should be more than zero");
    }

    if num_threads > numbers.len() {
        panic!("The number of threads cannot be more than the number of elements");
    }

    let size_per_thread = (numbers.len() - 1) / num_threads + 1;
    let arc_nums = Arc::new(numbers);
    let mut threads = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let nums = arc_nums.clone();

        let thread = spawn(move || {
            let from = i * size_per_thread;
            let to = min(from + size_per_thread, nums.len());

            let mut local_sum = T::default();
            for num in &nums[from..to] {
                local_sum += *num;
            }

            local_sum
        });

        threads.push(thread);
    }

    let mut global_sum = T::default();
    for t in threads {
        global_sum += t.join().expect("panic in thread");
    }
    global_sum
}
