use std::cmp::min;
use std::ops::AddAssign;
use std::sync::Arc;
use std::thread::spawn;

fn concurrent_sum<T>(numbers: Vec<T>, num_threads: usize) -> T
where
    T: AddAssign + Copy + Default + Send + Sync + 'static,
{
    if num_threads == 0 || num_threads > numbers.len() {
        panic!(
            "The number of threads should be between [1, {}], given: {}",
            numbers.len(),
            num_threads
        );
    }

    if numbers.is_empty() {
        return T::default();
    }

    let size_per_thread = (numbers.len() - 1) / num_threads + 1;
    let arc_nums = Arc::new(numbers);
    let mut threads = Vec::with_capacity(num_threads);

    for i in 0..num_threads {
        let nums = arc_nums.clone();

        let thread = spawn(move || {
            let from = i * size_per_thread;
            let to = min(from + size_per_thread, nums.len());

            let mut sum = T::default();
            for num in &nums[from..to] {
                sum += *num;
            }

            sum
        });

        threads.push(thread);
    }

    let mut sum = T::default();
    for t in threads {
        sum += t.join().expect("panic in thread");
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_valid() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sequential: u64 = vec.iter().cloned().sum();
        assert_eq!(sequential, concurrent_sum(vec, 2));
    }

    #[test]
    #[should_panic]
    fn sum_panic_more_threads() {
        let vec = vec![1];
        _ = concurrent_sum(vec, 2);
    }

    #[test]
    #[should_panic]
    fn sum_panic_zero_threads() {
        let vec = vec![1];
        _ = concurrent_sum(vec, 0);
    }
}
