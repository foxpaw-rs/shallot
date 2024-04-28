//! Json module which houses the Json deserializer.

use crate::deserialize::{Deserialize, Deserializer};

/// Json deserializer which converts JSON strings into deserialize items.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json;

impl Json {
    /// Create a new Json deserializer.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialize::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Deserializer for Json {
    /// The input type for this Deserializer.
    /// Todo(Paul): This should accept a &str.
    type Input = String;

    /// Deserialize the input into the required output type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<(), i8> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null".to_owned())?;
    ///     Ok(())
    /// }
    /// ```
    fn deserialize<S>(&self, input: &Self::Input) -> Result<S, i8>
    where
        S: Deserialize,
    {
        S::accept(self, input)
    }

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<(), i8> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null".to_owned())?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_unit(&self, input: &Self::Input) -> Result<(), i8> {
        if input == "null" {
            Ok(())
        } else {
            Err(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Json::new creates a Json as expected.
    #[test]
    fn json_new_correct() {
        let expected = Json {};
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly deserializes a unit type.
    #[test]
    fn json_visit_unit_correct() {
        let expected = Ok(());
        let actual = Json::new().deserialize(&"null".to_owned());
        assert_eq!(expected, actual);
    }
}
