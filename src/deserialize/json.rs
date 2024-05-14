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

    /// Convert a ParseIntError into a library error type.
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

    /// Visit and deserialize an i8 type.
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
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<i8>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i8"))
    }

    /// Visit and deserialize an i16 type.
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
    ///     let output: i16 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i16(&self, input: &Self::Input) -> Result<i16> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<i16>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i16"))
    }

    /// Visit and deserialize an i32 type.
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
    ///     let output: i32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i32(&self, input: &Self::Input) -> Result<i32> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<i32>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i32"))
    }

    /// Visit and deserialize an i64 type.
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
    ///     let output: i64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i64(&self, input: &Self::Input) -> Result<i64> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<i64>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i64"))
    }

    /// Visit and deserialize an i128 type.
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
    ///     let output: i128 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i128(&self, input: &Self::Input) -> Result<i128> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<i128>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "i128"))
    }

    /// Visit and deserialize an isize type.
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
    ///     let output: isize = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_isize(&self, input: &Self::Input) -> Result<isize> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<isize>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "isize"))
    }

    /// Visit and deserialize an u8 type.
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
    ///     let output: u8 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u8(&self, input: &Self::Input) -> Result<u8> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<u8>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "u8"))
    }

    /// Visit and deserialize an u16 type.
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
    ///     let output: u16 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u16(&self, input: &Self::Input) -> Result<u16> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<u16>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "u16"))
    }

    /// Visit and deserialize an u32 type.
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
    ///     let output: u32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u32(&self, input: &Self::Input) -> Result<u32> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<u32>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "u32"))
    }

    /// Visit and deserialize an u64 type.
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
    ///     let output: u64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u64(&self, input: &Self::Input) -> Result<u64> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<u64>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "u64"))
    }

    /// Visit and deserialize an u128 type.
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
    ///     let output: u128 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u128(&self, input: &Self::Input) -> Result<u128> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<u128>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "u128"))
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

    /// Visit and deserialize an usize type.
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
    ///     let output: usize = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_usize(&self, input: &Self::Input) -> Result<usize> {
        if input.starts_with("0") && input.len() > 1 {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected("0")
                .expected("1-9")
                .into())
        }

        input.parse::<usize>().map_err(|err: ParseIntError| self.convert_int_error(err, input, "usize"))
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
        let value = i8::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().visit_i8(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon negative overflow.
    #[test]
    fn visit_i8_negative_overflow() {
        let value = i8::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().visit_i8(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes an i16 type.
    #[test]
    fn visit_i16_positive() {
        let expected = Ok(1_i16);
        let actual = Json::new().visit_i16(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes a negative i16 type.
    #[test]
    fn visit_i16_negative() {
        let expected = Ok(-1_i16);
        let actual = Json::new().visit_i16(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes a zero i16 type.
    #[test]
    fn visit_i16_zero() {
        let expected = Ok(0_i16);
        let actual = Json::new().visit_i16(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon empty value.
    #[test]
    fn visit_i16_empty() {
        let expected = Err(Syntax::new(1, 1).expected("i16").into());
        let actual = Json::new().visit_i16(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon a leading zero.
    #[test]
    fn visit_i16_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_i16(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid character.
    #[test]
    fn visit_i16_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i16(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i16_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_i16(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid newline.
    #[test]
    fn visit_i16_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_i16(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon overflow.
    #[test]
    fn visit_i16_overflow() {
        let value = i16::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i16").into());
        let actual = Json::new().visit_i16(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon negative overflow.
    #[test]
    fn visit_i16_negative_overflow() {
        let value = i16::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i16").into());
        let actual = Json::new().visit_i16(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes an i32 type.
    #[test]
    fn visit_i32_positive() {
        let expected = Ok(1_i32);
        let actual = Json::new().visit_i32(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes a negative i32 type.
    #[test]
    fn visit_i32_negative() {
        let expected = Ok(-1_i32);
        let actual = Json::new().visit_i32(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes a zero i32 type.
    #[test]
    fn visit_i32_zero() {
        let expected = Ok(0_i32);
        let actual = Json::new().visit_i32(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon empty value.
    #[test]
    fn visit_i32_empty() {
        let expected = Err(Syntax::new(1, 1).expected("i32").into());
        let actual = Json::new().visit_i32(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon a leading zero.
    #[test]
    fn visit_i32_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_i32(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid character.
    #[test]
    fn visit_i32_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i32(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i32_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_i32(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid newline.
    #[test]
    fn visit_i32_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_i32(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon overflow.
    #[test]
    fn visit_i32_overflow() {
        let value = i32::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i32").into());
        let actual = Json::new().visit_i32(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon negative overflow.
    #[test]
    fn visit_i32_negative_overflow() {
        let value = i32::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i32").into());
        let actual = Json::new().visit_i32(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes an i64 type.
    #[test]
    fn visit_i64_positive() {
        let expected = Ok(1_i64);
        let actual = Json::new().visit_i64(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes a negative i64 type.
    #[test]
    fn visit_i64_negative() {
        let expected = Ok(-1_i64);
        let actual = Json::new().visit_i64(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes a zero i64 type.
    #[test]
    fn visit_i64_zero() {
        let expected = Ok(0_i64);
        let actual = Json::new().visit_i64(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon empty value.
    #[test]
    fn visit_i64_empty() {
        let expected = Err(Syntax::new(1, 1).expected("i64").into());
        let actual = Json::new().visit_i64(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon a leading zero.
    #[test]
    fn visit_i64_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_i64(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid character.
    #[test]
    fn visit_i64_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i64(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i64_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_i64(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid newline.
    #[test]
    fn visit_i64_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_i64(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon overflow.
    #[test]
    fn visit_i64_overflow() {
        let value = i64::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i64").into());
        let actual = Json::new().visit_i64(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon negative overflow.
    #[test]
    fn visit_i64_negative_overflow() {
        let value = i64::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i64").into());
        let actual = Json::new().visit_i64(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes an i128 type.
    #[test]
    fn visit_i128_positive() {
        let expected = Ok(1_i128);
        let actual = Json::new().visit_i128(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes a negative i128 type.
    #[test]
    fn visit_i128_negative() {
        let expected = Ok(-1_i128);
        let actual = Json::new().visit_i128(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes a zero i128 type.
    #[test]
    fn visit_i128_zero() {
        let expected = Ok(0_i128);
        let actual = Json::new().visit_i128(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon empty value.
    #[test]
    fn visit_i128_empty() {
        let expected = Err(Syntax::new(1, 1).expected("i128").into());
        let actual = Json::new().visit_i128(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon a leading zero.
    #[test]
    fn visit_i128_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_i128(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid character.
    #[test]
    fn visit_i128_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i128(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i128_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_i128(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid newline.
    #[test]
    fn visit_i128_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_i128(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon overflow.
    #[test]
    fn visit_i128_overflow() {
        let value = i128::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i128").into());
        let actual = Json::new().visit_i128(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon negative overflow.
    #[test]
    fn visit_i128_negative_overflow() {
        let value = i128::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("i128").into());
        let actual = Json::new().visit_i128(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes an isize type.
    #[test]
    fn visit_isize_positive() {
        let expected = Ok(1_isize);
        let actual = Json::new().visit_isize(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes a negative isize type.
    #[test]
    fn visit_isize_negative() {
        let expected = Ok(-1_isize);
        let actual = Json::new().visit_isize(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes a zero isize type.
    #[test]
    fn visit_isize_zero() {
        let expected = Ok(0_isize);
        let actual = Json::new().visit_isize(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon empty value.
    #[test]
    fn visit_isize_empty() {
        let expected = Err(Syntax::new(1, 1).expected("isize").into());
        let actual = Json::new().visit_isize(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon a leading zero.
    #[test]
    fn visit_isize_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_isize(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid character.
    #[test]
    fn visit_isize_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_isize(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid whitespace.
    #[test]
    fn visit_isize_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_isize(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid newline.
    #[test]
    fn visit_isize_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_isize(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon overflow.
    #[test]
    fn visit_isize_overflow() {
        let value = i128::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("isize").into());
        let actual = Json::new().visit_isize(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon negative overflow.
    #[test]
    fn visit_isize_negative_overflow() {
        let value = i128::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("isize").into());
        let actual = Json::new().visit_isize(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly deserializes an u8 type.
    #[test]
    fn visit_u8_positive() {
        let expected = Ok(1_u8);
        let actual = Json::new().visit_u8(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly deserializes a zero u8 type.
    #[test]
    fn visit_u8_zero() {
        let expected = Ok(0_u8);
        let actual = Json::new().visit_u8(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon empty value.
    #[test]
    fn visit_u8_empty() {
        let expected = Err(Syntax::new(1, 1).expected("u8").into());
        let actual = Json::new().visit_u8(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon a leading zero.
    #[test]
    fn visit_u8_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_u8(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid character.
    #[test]
    fn visit_u8_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_u8(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u8_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_u8(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid newline.
    #[test]
    fn visit_u8_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_u8(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon overflow.
    #[test]
    fn visit_u8_overflow() {
        let value = u8::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("u8").into());
        let actual = Json::new().visit_u8(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon negative values.
    #[test]
    fn visit_u8_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_u8(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly deserializes an u16 type.
    #[test]
    fn visit_u16_positive() {
        let expected = Ok(1_u16);
        let actual = Json::new().visit_u16(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly deserializes a zero u16 type.
    #[test]
    fn visit_u16_zero() {
        let expected = Ok(0_u16);
        let actual = Json::new().visit_u16(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon empty value.
    #[test]
    fn visit_u16_empty() {
        let expected = Err(Syntax::new(1, 1).expected("u16").into());
        let actual = Json::new().visit_u16(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon a leading zero.
    #[test]
    fn visit_u16_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_u16(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid character.
    #[test]
    fn visit_u16_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_u16(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u16_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_u16(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid newline.
    #[test]
    fn visit_u16_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_u16(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon overflow.
    #[test]
    fn visit_u16_overflow() {
        let value = u16::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("u16").into());
        let actual = Json::new().visit_u16(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon negative values.
    #[test]
    fn visit_u16_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_u16(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly deserializes an u32 type.
    #[test]
    fn visit_u32_positive() {
        let expected = Ok(1_u32);
        let actual = Json::new().visit_u32(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly deserializes a zero u32 type.
    #[test]
    fn visit_u32_zero() {
        let expected = Ok(0_u32);
        let actual = Json::new().visit_u32(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon empty value.
    #[test]
    fn visit_u32_empty() {
        let expected = Err(Syntax::new(1, 1).expected("u32").into());
        let actual = Json::new().visit_u32(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon a leading zero.
    #[test]
    fn visit_u32_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_u32(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid character.
    #[test]
    fn visit_u32_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_u32(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u32_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_u32(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid newline.
    #[test]
    fn visit_u32_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_u32(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon overflow.
    #[test]
    fn visit_u32_overflow() {
        let value = u32::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("u32").into());
        let actual = Json::new().visit_u32(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon negative values.
    #[test]
    fn visit_u32_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_u32(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly deserializes an u64 type.
    #[test]
    fn visit_u64_positive() {
        let expected = Ok(1_u64);
        let actual = Json::new().visit_u64(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly deserializes a zero u64 type.
    #[test]
    fn visit_u64_zero() {
        let expected = Ok(0_u64);
        let actual = Json::new().visit_u64(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon empty value.
    #[test]
    fn visit_u64_empty() {
        let expected = Err(Syntax::new(1, 1).expected("u64").into());
        let actual = Json::new().visit_u64(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon a leading zero.
    #[test]
    fn visit_u64_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_u64(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid character.
    #[test]
    fn visit_u64_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_u64(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u64_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_u64(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid newline.
    #[test]
    fn visit_u64_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_u64(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon overflow.
    #[test]
    fn visit_u64_overflow() {
        let value = u64::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("u64").into());
        let actual = Json::new().visit_u64(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon negative values.
    #[test]
    fn visit_u64_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_u64(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly deserializes an u128 type.
    #[test]
    fn visit_u128_positive() {
        let expected = Ok(1_u128);
        let actual = Json::new().visit_u128(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly deserializes a zero u128 type.
    #[test]
    fn visit_u128_zero() {
        let expected = Ok(0_u128);
        let actual = Json::new().visit_u128(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon empty value.
    #[test]
    fn visit_u128_empty() {
        let expected = Err(Syntax::new(1, 1).expected("u128").into());
        let actual = Json::new().visit_u128(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon a leading zero.
    #[test]
    fn visit_u128_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_u128(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid character.
    #[test]
    fn visit_u128_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_u128(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u128_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_u128(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid newline.
    #[test]
    fn visit_u128_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_u128(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon overflow.
    #[test]
    fn visit_u128_overflow() {
        let value = u128::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("u128").into());
        let actual = Json::new().visit_u128(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon negative values.
    #[test]
    fn visit_u128_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_u128(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
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

    /// Test Json::visit_usize correctly deserializes an usize type.
    #[test]
    fn visit_usize_positive() {
        let expected = Ok(1_usize);
        let actual = Json::new().visit_usize(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly deserializes a zero usize type.
    #[test]
    fn visit_usize_zero() {
        let expected = Ok(0_usize);
        let actual = Json::new().visit_usize(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon empty value.
    #[test]
    fn visit_usize_empty() {
        let expected = Err(Syntax::new(1, 1).expected("usize").into());
        let actual = Json::new().visit_usize(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon a leading zero.
    #[test]
    fn visit_usize_leading_zero() {
        let expected = Err(Syntax::new(1, 1).unexpected("0").expected("1-9").into());
        let actual = Json::new().visit_usize(&"01");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"01");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid character.
    #[test]
    fn visit_usize_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_usize(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid whitespace.
    #[test]
    fn visit_usize_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_usize(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid newline.
    #[test]
    fn visit_usize_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_usize(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon overflow.
    #[test]
    fn visit_usize_overflow() {
        let value = u128::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("usize").into());
        let actual = Json::new().visit_usize(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon negative values.
    #[test]
    fn visit_usize_negative() {
        let expected = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().visit_usize(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }
}
