//! Json module which houses the Json deserializer.

use crate::deserialize::{Deserialize, Deserializer};
use crate::error::{Error, Overflow, Result, Syntax};
use std::cell::Cell;
use std::marker::PhantomData;
use std::num::{IntErrorKind, ParseIntError};

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

    /// Convert a ParseIntError into an error type.
    fn convert_int_error(&self, err: ParseIntError, input: &<Self as Deserializer>::Input, kind: &str) -> Error {
        match err.kind() {
            IntErrorKind::Empty => Syntax::new(self.row.get(), self.col.get())
                .expected(kind)
                .into(),
            IntErrorKind::InvalidDigit => {
                for c in input.chars() {
                    match c {
                        '0'..='9' => self.col.set(self.col.get() + 1),
                        c if c.is_whitespace() => {
                            if c == '\r' || c == '\n' {
                                self.row.set(self.row.get() + 1);
                                self.col.set(1);
                            }
                            return Syntax::new(self.row.get(), self.col.get())
                                .unexpected("whitespace")
                                .into();
                        }
                        _ => {
                            return Syntax::new(self.row.get(), self.col.get())
                                .unexpected(&c.to_string())
                                .into()
                        }
                    }
                }
                Syntax::new(self.row.get(), self.col.get()).into()
            }
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                Overflow::new(self.row.get(), self.col.get())
                    .kind(kind)
                    .into()
            }
            _ => Syntax::new(self.row.get(), self.col.get()).into(),
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

    /// Visit and deserialize a i8 type.
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
    ///     let output: i8 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i8(&self, input: &Self::Input) -> Result<i8> {
        let mut chars = input.chars();
        if chars.next() == Some('0') {
            return if chars.next().is_none() {
                Ok(0)
            } else {
                Err(Syntax::new(self.row.get(), self.col.get())
                    .unexpected("0")
                    .expected("1-9")
                    .into())
            };
        }

        input.parse::<i8>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i8"))
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

    /// Test Json::visit_i8 correctly deserializes an i8 type.
    #[test]
    fn visit_i8_positive() {
        let expected = Ok(1_i8);
        let actual = Json::new().visit_i8(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes a negative i8 type.
    #[test]
    fn visit_i8_negative() {
        let expected = Ok(-1_i8);
        let actual = Json::new().visit_i8(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes a zero i8 type.
    #[test]
    fn visit_i8_zero() {
        let expected = Ok(0_i8);
        let actual = Json::new().visit_i8(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon empty value.
    #[test]
    fn visit_i8_empty() {
        let expected = Err(Syntax::new(1, 1).expected("i8").into());
        let actual = Json::new().visit_i8(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon a leading zero.
    #[test]
    fn visit_i8_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_i8(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid character.
    #[test]
    fn visit_i8_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i8(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i8_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_i8(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid newline.
    #[test]
    fn visit_i8_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_i8(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon overflow.
    #[test]
    fn visit_i8_overflow() {
        let expected = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().visit_i8(&"128");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"128");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon negative overflow.
    #[test]
    fn visit_i8_negative_overflow() {
        let expected = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().visit_i8(&"-129");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-129");
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
}
