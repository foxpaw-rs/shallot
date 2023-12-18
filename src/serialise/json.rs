//! Json
//!
//! Json module which houses the Json serialiser.

use crate::serialise::{Serialise, Serialiser};

/// Json
///
/// Json serialiser, will serialise data into Json formatting.
///
/// # Examples
/// ```rust
/// use shallot::serialise::{Json, Serialiser};
///
/// let output = Json::new().serialise(&());
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Json;

impl Json {
    /// Create a new Json.
    ///
    /// Creates a new Json serialiser.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialise::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Serialiser for Json {
    /// The output type for a Json serialiser.
    type Output = String;

    /// Serialise the input.
    ///
    /// Request this serialiser serialise the provided input.
    fn serialise<T>(&self, input: &T) -> Self::Output
    where
        T: Serialise,
    {
        input.accept(self)
    }

    /// Visit a unit type.
    ///
    /// Visit and serialise a unit type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialise::{Json, Serialiser};
    ///
    /// let serialised = Json::new().visit_unit();
    /// ```
    fn visit_unit(&self) -> Self::Output {
        "()".to_owned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Json::new() creates a Json as expected.
    ///
    /// Test that Json::new() creates a new Json as expected.
    #[test]
    fn json_new() {
        let expected = Json {};
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit correctly serialises a unit.
    ///
    /// Test that Json::visit_unit correctly serialises a unit.
    #[test]
    fn json_visit_unit() {
        let expected = "()".to_owned();
        let actual = Json::new().visit_unit();
        assert_eq!(expected, actual);

        let actual = Json::new().serialise(&());
        assert_eq!(expected, actual);
    }
}
