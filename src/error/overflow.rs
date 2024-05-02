//! Overflow module to house the overflow error type. Used to signify when an
//! overflow error has occurred
//!
//! # Examples
//! ```rust
//! use shallot::error::Overflow;
//!
//! let error = Overflow::new(1, 1).value("i8");
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

    /// What type of value overflowed.
    value: Option<String>,
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
            value: None,
        }
    }

    /// Set the type value, to notify the user what type overflowed.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Overflow;
    ///
    /// let error = Overflow::new(1, 1).value("i8");
    /// ```
    #[must_use]
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_owned());
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
    /// let error = Overflow::new(1, 1).value("i8");
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
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
            value: None,
        };
        let actual = Overflow::new(1, 1);
        assert_eq!(expected, actual);
    }

    /// Test Overflow::value sets the type value.
    #[test]
    fn value_correct() {
        let value = Some("i8".to_owned());
        let actual = Overflow::new(1, 1).value("i8").value;
        assert_eq!(value, actual);
    }

    /// Test Overflow::fmt formats with no set type value.
    #[test]
    fn fmt_none() {
        let expected = "Overflow error at (1, 1)".to_owned();
        let actual = Overflow::new(1, 1).to_string();
        assert_eq!(expected, actual);
    }

    /// Test Overflow::fmt formats with a type value.
    #[test]
    fn fmt_type() {
        let expected = "Overflow error for i8 type at (1, 1)".to_owned();
        let actual = Overflow::new(1, 1).value("i8").to_string();
        assert_eq!(expected, actual);
    }
}
