//! Json module which houses the Json serializer.

use crate::serialize::{Serialize, Serializer};

/// Json serializer which converts serialize items into JSON strings.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json;

impl Json {
    /// Create a new Json serializer.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Serializer for Json {
    type Output = String;

    /// Serialize the input into the required output type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&());
    /// ```
    fn serialize<S>(&self, input: &S) -> Self::Output
    where
        S: Serialize,
    {
        input.accept(self)
    }

    /// Visit and serialize a bool type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&true);
    /// ```
    fn visit_bool(&self, input: &bool) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize a f32 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_f32);
    /// ```
    fn visit_f32(&self, input: &f32) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize a f64 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_f64);
    /// ```
    fn visit_f64(&self, input: &f64) -> Self::Output {
        input.to_string()
    }


    /// Visit and serialize an i8 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_i8);
    /// ```
    fn visit_i8(&self, input: &i8) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an i16 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_i16);
    /// ```
    fn visit_i16(&self, input: &i16) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an i32 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_i32);
    /// ```
    fn visit_i32(&self, input: &i32) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an i64 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_i64);
    /// ```
    fn visit_i64(&self, input: &i64) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an i128 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_i128);
    /// ```
    fn visit_i128(&self, input: &i128) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an isize type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_isize);
    /// ```
    fn visit_isize(&self, input: &isize) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an u8 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_u8);
    /// ```
    fn visit_u8(&self, input: &u8) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an u16 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_u16);
    /// ```
    fn visit_u16(&self, input: &u16) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an u32 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_u32);
    /// ```
    fn visit_u32(&self, input: &u32) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an u64 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_u64);
    /// ```
    fn visit_u64(&self, input: &u64) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize an u128 type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&1_u128);
    /// ```
    fn visit_u128(&self, input: &u128) -> Self::Output {
        input.to_string()
    }

    /// Visit and serialize a unit type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&());
    /// ```
    fn visit_unit(&self) -> Self::Output {
        "null".to_owned()
    }

    /// Visit and serialize an usize type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialize::{Json, Serializer};
    ///
    /// let json = Json::new();
    /// let output = json.serialize(&true);
    /// ```
    fn visit_usize(&self, input: &usize) -> Self::Output {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Json::new creates a Json as expected.
    #[test]
    fn new_correct() {
        let expected = Json {};
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly serializes a true bool type.
    #[test]
    fn visit_bool_true() {
        let expected = "true".to_owned();
        let actual = Json::new().visit_bool(&true);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&true);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly serializes a false bool type.
    #[test]
    fn visit_bool_false() {
        let expected = "false".to_owned();
        let actual = Json::new().visit_bool(&false);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&false);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly serializes an f32 type.
    #[test]
    fn visit_f32_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_f32(&1_f32);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_f32);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly serializes an f64 type.
    #[test]
    fn visit_f64_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_f64(&1_f64);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_f64);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly serializes an i8 type.
    #[test]
    fn visit_i8_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_i8(&1_i8);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_i8);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly serializes an i16 type.
    #[test]
    fn visit_i16_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_i16(&1_i16);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_i16);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly serializes an i32 type.
    #[test]
    fn visit_i32_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_i32(&1_i32);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_i32);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly serializes an i64 type.
    #[test]
    fn visit_i64_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_i64(&1_i64);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_i64);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly serializes an i128 type.
    #[test]
    fn visit_i128_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_i128(&1_i128);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_i128);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly serializes an isize type.
    #[test]
    fn visit_isize_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_isize(&1_isize);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_isize);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly serializes a u8 type.
    #[test]
    fn visit_u8_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_u8(&1_u8);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_u8);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly serializes a u16 type.
    #[test]
    fn visit_u16_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_u16(&1_u16);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_u16);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly serializes a u32 type.
    #[test]
    fn visit_u32_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_u32(&1_u32);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_u32);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly serializes a u64 type.
    #[test]
    fn visit_u64_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_u64(&1_u64);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_u64);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly serializes a u128 type.
    #[test]
    fn visit_u128_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_u128(&1_u128);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_u128);
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly serializes a unit type.
    #[test]
    fn visit_unit_correct() {
        let expected = "null".to_owned();
        let actual = Json::new().visit_unit();
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly serializes a usize type.
    #[test]
    fn visit_usize_correct() {
        let expected = "1".to_owned();
        let actual = Json::new().visit_usize(&1_usize);
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&1_usize);
        assert_eq!(expected, actual);
    }
}
