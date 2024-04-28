//! Json module which houses the Json serializer.

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Json::new() creates a Json as expected.
    #[test]
    fn json_new_correct() {
        let expected = Json {};
        let actual = Json::new();
        assert_eq!(expected, actual);
    }
}
