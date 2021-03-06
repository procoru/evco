//! Genetic Programming for Rust
//!
//! This crate implements a Genetic Programming library inspired by the GP in
//! [Python's DEAP](https://github.com/DEAP/deap). This library is hoped to be
//! more performant and use Rust's typesystem to obtain simpler code.
//!
//! Presently under [active development](https://github.com/46bit/evco).

// https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

extern crate rand;
// #[cfg(test)]
// extern crate quickcheck;

/// Genetic Programming.
pub mod gp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
