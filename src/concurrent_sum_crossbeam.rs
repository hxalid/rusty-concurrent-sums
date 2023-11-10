//! Concurrent vector sum implementation in Rust using Crossbeam crate.
//!

use std::ops::AddAssign;
use std::sync::mpsc;

fn concurrent_sum_crossbeam<T>(numbers: Vec<T>, num_threads: usize) -> T
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

    let (sender, receiver) = mpsc::channel::<T>();

    crossbeam::scope(|scope| {
        for chunk in numbers.chunks(size_per_thread) {
            let sender_thread = sender.clone();

            scope.spawn(move |_| {
                let mut local_sum = T::default();
                for num in chunk {
                    local_sum += *num;
                }

                sender_thread.send(local_sum).unwrap();
            });
        }

        drop(sender);

        let mut global_sum = T::default();
        for t in receiver.iter() {
            global_sum += t;
        }

        global_sum
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_valid() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let sequential: u64 = vec.iter().cloned().sum();
        assert_eq!(sequential, concurrent_sum_crossbeam(vec, 2));
    }

    #[test]
    #[should_panic]
    fn sum_panic_more_threads() {
        let vec = vec![1];
        _ = concurrent_sum_crossbeam(vec, 2);
    }

    #[test]
    #[should_panic]
    fn sum_panic_zero_threads() {
        let vec = vec![1];
        _ = concurrent_sum_crossbeam(vec, 0);
    }
}
