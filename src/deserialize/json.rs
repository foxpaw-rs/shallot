//! Json module which houses the Json deserializer.

use crate::deserialize::{Deserialize, Deserializer};
use crate::error::{Result, Syntax};
use std::cell::Cell;
use std::marker::PhantomData;

/// Json deserializer which converts JSON strings into deserialize items.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json<'a> {
    /// The current column number.
    col: Cell<usize>,

    /// The current row number.
    row: Cell<usize>,

    /// Phantomdata to hold the lifetime of the Input &str.
    phantom: PhantomData<&'a ()>,
}

impl<'a> Json<'a> {
    /// Create a new Json deserializer.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            col: Cell::new(1),
            row: Cell::new(1),
            phantom: PhantomData,
        }
    }
}

impl<'a> Deserializer for Json<'a> {
    /// The input type for this Deserializer.
    type Input = &'a str;

    /// Deserialize the input into the required output type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null")?;
    ///     Ok(())
    /// }
    /// ```
    fn deserialize<S>(&self, input: &Self::Input) -> Result<S>
    where
        S: Deserialize,
    {
        S::accept(self, input)
    }

    /// Visit and deserialize a bool type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: bool = json.deserialize(&"true")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_bool(&self, input: &Self::Input) -> Result<bool> {
        match *input {
            "true" => {
                self.col.set(self.col.get() + 4);
                Ok(true)
            }
            "false" => {
                self.col.set(self.col.get() + 5);
                Ok(false)
            }
            _ => Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(input)
                .expected("boolean")
                .into()),
        }
    }

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_unit(&self, input: &Self::Input) -> Result<()> {
        if *input == "null" {
            self.col.set(self.col.get() + 4);
            Ok(())
        } else {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(input)
                .expected("null")
                .into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Json::new creates a Json as expected.
    #[test]
    fn new_correct() {
        let expected = Json {
            col: Cell::new(1),
            row: Cell::new(1),
            phantom: PhantomData,
        };
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly deserializes a unit type.
    #[test]
    fn visit_unit_correct() {
        let expected = Ok(());
        let actual = Json::new().visit_unit(&"null");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"null");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly errors upon unexpected value.
    #[test]
    fn visit_unit_incorrect() {
        let expected = Err(Syntax::new(1, 1).unexpected("fail").expected("null").into());
        let actual = Json::new().visit_unit(&"fail");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"fail");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly deserializes a true bool type.
    #[test]
    fn visit_bool_true() {
        let expected = Ok(true);
        let actual = Json::new().visit_bool(&"true");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"true");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly deserializes a false bool type.
    #[test]
    fn visit_bool_false() {
        let expected = Ok(false);
        let actual = Json::new().visit_bool(&"false");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"false");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly errors upon unexpected value.
    #[test]
    fn visit_bool_incorrect() {
        let expected = Err(Syntax::new(1, 1)
            .unexpected("fail")
            .expected("boolean")
            .into());
        let actual = Json::new().visit_bool(&"fail");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"fail");
        assert_eq!(expected, actual);
    }
}
