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

    /// Test Json::visit_unit correctly serializes a unit type.
    #[test]
    fn visit_unit_correct() {
        let expected = "null".to_owned();
        let actual = Json::new().visit_unit();
        assert_eq!(expected, actual);

        let actual = Json::new().serialize(&());
        assert_eq!(expected, actual);
    }
}
