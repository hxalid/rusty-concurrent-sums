#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use rusty_concurrent_sums::concurrent_sum_std::concurrent_sum;

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
