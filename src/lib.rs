//! Shallot serialization library to serialize and deserialize data.
//!
//! # Supports
//! ## Primitives
//! * ()

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
pub mod serialize;
