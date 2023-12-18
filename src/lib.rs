//! Shallot
//!
//! Shallot serialisation library to serialise and deserialise data.
//!
//! # Supports
//! ## Primitives
//! * ()

#![deny(
     clippy::all,
     // clippy::cargo,
     clippy::complexity,
     clippy::correctness,
     clippy::missing_docs_in_private_items,
     clippy::nursery,
     clippy::pedantic,
     clippy::perf,
     clippy::style,
     clippy::suspicious,
 )]

pub mod error;
pub mod serialise;
