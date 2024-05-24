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

    /// Convert a float errors into library error types.
    fn convert_float_error(&self, input: &<Self as Deserializer>::Input, kind: &str) -> Error {
        let mut first = true;
        let mut dot = false;
        for c in input.chars() {
            match c {
                '0'..='9' => self.col.set(self.col.get() + 1),
                '-' if first => self.col.set(self.col.get() + 1),
                '.' if !dot => {
                    self.col.set(self.col.get() + 1);
                    dot = true;
                }
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

            if first {
                first = false;
            }
        }

        Syntax::new(self.row.get(), self.col.get())
            .expected(kind)
            .into()
    }

    /// Convert a integer errors into library error types.
    fn convert_int_error(
        &self,
        err: &ParseIntError,
        input: &<Self as Deserializer>::Input,
        kind: &str,
    ) -> Error {
        match err.kind() {
            IntErrorKind::Empty => Syntax::new(self.row.get(), self.col.get())
                .expected(kind)
                .into(),
            IntErrorKind::InvalidDigit => {
                let mut first = true;
                for c in input.chars() {
                    match c {
                        '0'..='9' => self.col.set(self.col.get() + 1),
                        '-' if first && kind.starts_with('i') => self.col.set(self.col.get() + 1),
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

                    if first {
                        first = false;
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

    /// Update the row and col values.
    fn update(&self, input: &<Self as Deserializer>::Input) -> Result<&Self> {
        if input.contains('\n') {
            let parts: Vec<_> = input.split('\n').collect();
            self.row.set(self.row.get() + parts.len() - 1);
            self.col.set(
                parts
                    .last()
                    .ok_or_else(|| Syntax::new(self.row.get(), self.col.get()))?
                    .len(),
            );
        } else {
            self.col.set(self.col.get() + input.len());
        }
        Ok(self)
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
        let result = S::accept(self, input)?;
        self.update(input)?;
        Ok(result)
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
        match input.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(input)
                .expected("boolean")
                .into()),
        }
    }

    /// Visit and deserialize an f32 type.
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
    ///     let output: f32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_f32(&self, input: &Self::Input) -> Result<f32> {
        let result = input
            .trim()
            .parse::<f32>()
            .map_err(|_| self.convert_float_error(input, "f32"))?;
        if result.is_finite() {
            Ok(result)
        } else {
            Err(Overflow::new(self.row.get(), self.col.get())
                .kind("f32")
                .into())
        }
    }

    /// Visit and deserialize an f64 type.
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
    ///     let output: f64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_f64(&self, input: &Self::Input) -> Result<f64> {
        let result = input
            .trim()
            .parse::<f64>()
            .map_err(|_| self.convert_float_error(input, "f64"))?;
        if result.is_finite() {
            Ok(result)
        } else {
            Err(Overflow::new(self.row.get(), self.col.get())
                .kind("f64")
                .into())
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
        input
            .trim()
            .parse::<i8>()
            .map_err(|err| self.convert_int_error(&err, input, "i8"))
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
        input
            .trim()
            .parse::<i16>()
            .map_err(|err| self.convert_int_error(&err, input, "i16"))
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
        input
            .trim()
            .parse::<i32>()
            .map_err(|err| self.convert_int_error(&err, input, "i32"))
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
        input
            .trim()
            .parse::<i64>()
            .map_err(|err| self.convert_int_error(&err, input, "i64"))
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
        input
            .trim()
            .parse::<i128>()
            .map_err(|err| self.convert_int_error(&err, input, "i128"))
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
        input
            .trim()
            .parse::<isize>()
            .map_err(|err| self.convert_int_error(&err, input, "isize"))
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
        input
            .trim()
            .parse::<u8>()
            .map_err(|err| self.convert_int_error(&err, input, "u8"))
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
        input
            .trim()
            .parse::<u16>()
            .map_err(|err| self.convert_int_error(&err, input, "u16"))
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
        input
            .trim()
            .parse::<u32>()
            .map_err(|err| self.convert_int_error(&err, input, "u32"))
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
        input
            .trim()
            .parse::<u64>()
            .map_err(|err| self.convert_int_error(&err, input, "u64"))
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
        input
            .trim()
            .parse::<u128>()
            .map_err(|err| self.convert_int_error(&err, input, "u128"))
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
        if input.trim() == "null" {
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
        input
            .trim()
            .parse::<usize>()
            .map_err(|err| self.convert_int_error(&err, input, "usize"))
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

    /// Test Json::visit_bool correctly deserializes with whitespace.
    #[test]
    fn visit_bool_whitespace() {
        let expected = Ok(false);
        let actual = Json::new().visit_bool(&" \nfalse  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \nfalse  ");
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

    /// Test Json::visit_f32 correctly deserializes an f32 type.
    #[test]
    fn visit_f32_positive() {
        let expected = Ok(1_f32);
        let actual = Json::new().visit_f32(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes a negative f32 type.
    #[test]
    fn visit_f32_negative() {
        let expected = Ok(-1_f32);
        let actual = Json::new().visit_f32(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes a zero f32 type.
    #[test]
    fn visit_f32_zero() {
        let expected = Ok(0_f32);
        let actual = Json::new().visit_f32(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes an f32 with surrounding whitespace.
    #[test]
    fn visit_f32_surrounding_whitespace() {
        let expected = Ok(0_f32);
        let actual = Json::new().visit_f32(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon empty value.
    #[test]
    fn visit_f32_empty() {
        let expected = Err(Syntax::new(1, 1).expected("f32").into());
        let actual = Json::new().visit_f32(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid character.
    #[test]
    fn visit_f32_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected("|").into());
        let actual = Json::new().visit_f32(&"1|2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1|2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid negative.
    #[test]
    fn visit_f32_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_f32(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid dot.
    #[test]
    fn visit_f32_invalid_dot() {
        let expected = Err(Syntax::new(1, 3).unexpected(".").into());
        let actual = Json::new().visit_f32(&".1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&".1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_f32_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_f32(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid newline.
    #[test]
    fn visit_f32_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_f32(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon overflow.
    #[test]
    fn visit_f32_overflow() {
        let value = f32::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("f32").into());
        let actual = Json::new().visit_f32(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon negative overflow.
    #[test]
    fn visit_f32_negative_overflow() {
        let value = f32::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("f32").into());
        let actual = Json::new().visit_f32(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes an f64 type.
    #[test]
    fn visit_f64_positive() {
        let expected = Ok(1_f64);
        let actual = Json::new().visit_f64(&"1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes a negative f64 type.
    #[test]
    fn visit_f64_negative() {
        let expected = Ok(-1_f64);
        let actual = Json::new().visit_f64(&"-1");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes a zero f64 type.
    #[test]
    fn visit_f64_zero() {
        let expected = Ok(0_f64);
        let actual = Json::new().visit_f64(&"0");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes an f64 with surrounding whitespace.
    #[test]
    fn visit_f64_surrounding_whitespace() {
        let expected = Ok(0_f64);
        let actual = Json::new().visit_f64(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon empty value.
    #[test]
    fn visit_f64_empty() {
        let expected = Err(Syntax::new(1, 1).expected("f64").into());
        let actual = Json::new().visit_f64(&"");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid character.
    #[test]
    fn visit_f64_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected("|").into());
        let actual = Json::new().visit_f64(&"1|2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1|2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid negative.
    #[test]
    fn visit_f64_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_f64(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid dot.
    #[test]
    fn visit_f64_invalid_dot() {
        let expected = Err(Syntax::new(1, 3).unexpected(".").into());
        let actual = Json::new().visit_f64(&".1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&".1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_f64_invalid_whitespace() {
        let expected = Err(Syntax::new(1, 2).unexpected("whitespace").into());
        let actual = Json::new().visit_f64(&"1 2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid newline.
    #[test]
    fn visit_f64_invalid_newline() {
        let expected = Err(Syntax::new(2, 1).unexpected("whitespace").into());
        let actual = Json::new().visit_f64(&"1\n2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon overflow.
    #[test]
    fn visit_f64_overflow() {
        let value = f64::MAX.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("f64").into());
        let actual = Json::new().visit_f64(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon negative overflow.
    #[test]
    fn visit_f64_negative_overflow() {
        let value = f64::MIN.to_string() + "0";
        let expected = Err(Overflow::new(1, 1).kind("f64").into());
        let actual = Json::new().visit_f64(&value.as_str());
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&value.as_str());
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

    /// Test Json::visit_i8 correctly deserializes an i8 with surrounding whitespace.
    #[test]
    fn visit_i8_surrounding_whitespace() {
        let expected = Ok(0_i8);
        let actual = Json::new().visit_i8(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_i8 correctly errors upon an invalid character.
    #[test]
    fn visit_i8_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i8(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid negative.
    #[test]
    fn visit_i8_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_i8(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_i16 correctly deserializes an i16 with surrounding whitespace.
    #[test]
    fn visit_i16_surrounding_whitespace() {
        let expected = Ok(0_i16);
        let actual = Json::new().visit_i16(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_i16 correctly errors upon an invalid character.
    #[test]
    fn visit_i16_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i16(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid negative.
    #[test]
    fn visit_i16_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_i16(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_i32 correctly deserializes an i32 with surrounding whitespace.
    #[test]
    fn visit_i32_surrounding_whitespace() {
        let expected = Ok(0_i32);
        let actual = Json::new().visit_i32(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_i32 correctly errors upon an invalid character.
    #[test]
    fn visit_i32_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i32(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid negative.
    #[test]
    fn visit_i32_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_i32(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_i64 correctly deserializes an i64 with surrounding whitespace.
    #[test]
    fn visit_i64_surrounding_whitespace() {
        let expected = Ok(0_i64);
        let actual = Json::new().visit_i64(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_i64 correctly errors upon an invalid character.
    #[test]
    fn visit_i64_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i64(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid negative.
    #[test]
    fn visit_i64_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_i64(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_i128 correctly deserializes an i128 with surrounding whitespace.
    #[test]
    fn visit_i128_surrounding_whitespace() {
        let expected = Ok(0_i128);
        let actual = Json::new().visit_i128(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_i128 correctly errors upon an invalid character.
    #[test]
    fn visit_i128_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_i128(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid negative.
    #[test]
    fn visit_i128_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_i128(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_isize correctly deserializes an isize with surrounding whitespace.
    #[test]
    fn visit_isize_surrounding_whitespace() {
        let expected = Ok(0_isize);
        let actual = Json::new().visit_isize(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_isize correctly errors upon an invalid character.
    #[test]
    fn visit_isize_invalid_character() {
        let expected = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().visit_isize(&"1.2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid negative.
    #[test]
    fn visit_isize_invalid_negative() {
        let expected = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().visit_isize(&"-1-2");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&"-1-2");
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

    /// Test Json::visit_u8 correctly deserializes an u8 with surrounding whitespace.
    #[test]
    fn visit_u8_surrounding_whitespace() {
        let expected = Ok(0_u8);
        let actual = Json::new().visit_u8(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_u16 correctly deserializes an u16 with surrounding whitespace.
    #[test]
    fn visit_u16_surrounding_whitespace() {
        let expected = Ok(0_u16);
        let actual = Json::new().visit_u16(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_u32 correctly deserializes an u32 with surrounding whitespace.
    #[test]
    fn visit_u32_surrounding_whitespace() {
        let expected = Ok(0_u32);
        let actual = Json::new().visit_u32(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_u64 correctly deserializes an u64 with surrounding whitespace.
    #[test]
    fn visit_u64_surrounding_whitespace() {
        let expected = Ok(0_u64);
        let actual = Json::new().visit_u64(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_u128 correctly deserializes an u128 with surrounding whitespace.
    #[test]
    fn visit_u128_surrounding_whitespace() {
        let expected = Ok(0_u128);
        let actual = Json::new().visit_u128(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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

    /// Test Json::visit_unit correctly deserializes with whitespace.
    #[test]
    fn visit_unit_whitespace() {
        let expected = Ok(());
        let actual = Json::new().visit_unit(&" \nnull  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \nnull  ");
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

    /// Test Json::visit_usize correctly deserializes an usize with surrounding whitespace.
    #[test]
    fn visit_usize_surrounding_whitespace() {
        let expected = Ok(0_usize);
        let actual = Json::new().visit_usize(&" \n0  ");
        assert_eq!(expected, actual);

        let actual = Json::new().deserialize(&" \n0  ");
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
