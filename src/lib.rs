#![allow(dead_code)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

//! Concurrent Vector sum implementation in Rust
//! using different approaches and crates.

#[cfg(feature = "crossbeam")]
pub mod concurrent_sum_crossbeam;

#[cfg(feature = "rayon")]
pub mod concurrent_sum_rayon;

#[cfg(feature = "std")]
pub mod concurrent_sum_std;
