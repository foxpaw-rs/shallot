//! Shallot serialization library to serialize and deserialize data.
//!
//! # Supports
//! ## Primitives
//! * ()
//! * bool
//! * i8, i16, i32, i64, i128, isize
//! * u8, u16, u32, u64, u128, usize
//! * f32, f64
//! * char
//! * &str [^1], String
//!
//! [^1]: Serialization of &str slices only supported, deserialization not
//! supported.

#![deny(
    clippy::all,
    // clippy::cargo
    clippy::complexity,
    clippy::correctness,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
)]

pub mod deserialize;
pub mod error;
pub mod serialize;
