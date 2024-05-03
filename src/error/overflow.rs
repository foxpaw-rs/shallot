//! Overflow module to house the overflow error type. Used to signify when an
//! overflow error has occurred
//!
//! # Examples
//! ```rust
//! use shallot::error::Overflow;
//!
//! let error = Overflow::new(1, 1).kind("i8");
//! ```

use std::error::Error;
use std::fmt;

/// Overflow error to signify that an overflow was located.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Overflow {
    /// The column where the overflow error occurs.
    col: usize,

    /// The row where the overflow error occurs.
    row: usize,

    /// What kind of value overflowed.
    kind: Option<String>,
}

impl Overflow {
    /// Create a new Overflow error.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Overflow;
    ///
    /// let error = Overflow::new(1, 1);
    /// ```
    #[must_use]
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            col,
            row,
            kind: None,
        }
    }

    /// Set the type kind, to notify the user what type overflowed.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Overflow;
    ///
    /// let error = Overflow::new(1, 1).kind("i8");
    /// ```
    #[must_use]
    pub fn kind(mut self, kind: &str) -> Self {
        self.kind = Some(kind.to_owned());
        self
    }
}

impl fmt::Display for Overflow {
    /// Format the error for displaying.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Overflow;
    ///
    /// let error = Overflow::new(1, 1).kind("i8");
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            None => write!(f, "Overflow error at ({}, {})", self.row, self.col),
            Some(v) => write!(
                f,
                "Overflow error for {v} type at ({}, {})",
                self.row, self.col
            ),
        }
    }
}

impl Error for Overflow {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Overflow::new creates a Overflow as expected.
    #[test]
    fn new_correct() {
        let expected = Overflow {
            col: 1,
            row: 1,
            kind: None,
        };
        let actual = Overflow::new(1, 1);
        assert_eq!(expected, actual);
    }

    /// Test Overflow::kind sets the type kind.
    #[test]
    fn kind_correct() {
        let kind = Some("i8".to_owned());
        let actual = Overflow::new(1, 1).kind("i8").kind;
        assert_eq!(kind, actual);
    }

    /// Test Overflow::fmt formats with no set type kind.
    #[test]
    fn fmt_none() {
        let expected = "Overflow error at (1, 1)".to_owned();
        let actual = Overflow::new(1, 1).to_string();
        assert_eq!(expected, actual);
    }

    /// Test Overflow::fmt formats with a type kind.
    #[test]
    fn fmt_type() {
        let expected = "Overflow error for i8 type at (1, 1)".to_owned();
        let actual = Overflow::new(1, 1).kind("i8").to_string();
        assert_eq!(expected, actual);
    }
}
