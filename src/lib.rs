#![allow(dead_code)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

//! Concurrent Vector sum implementation in Rust
//! using different approaches and crates.

pub mod concurrent_sum_crossbeam;
pub mod concurrent_sum_rayon;
pub mod concurrent_sum_std;
