//! Serialise::Json Module
//!
//! Contains implementation for the Json Deerialiser.
use crate::deserialise::{Deserialise, Deserialiser, ParseError};
use std::marker::PhantomData;

/// Json.
///
/// Json deserialiser for the Shallot library. Will deserialise Deserialise
/// implementing structs into a JSON format. Note that if a Json object is not
/// immediately chained as shown in the example, the intended Deserialise type
/// `T` will likely need to be annotated.
///
/// # Examples
/// ```rust
/// use shallot::deserialise::{Json, Deserialiser, ParseError};
///
/// fn main() -> Result<(), ParseError> {
///     let content = "1";
///     let deserialiser = Json::new(content);
///     let data: u8 = deserialiser.deserialise()?;
///     Ok(())
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json<'c, T>  where T: Deserialise {
    /// The content to serialise.
    content: &'c str,

    /// The PhantonData housing the type to deserialise into.
    phantom: PhantomData<T>,
}

impl<'c, T> Json<'c, T> where T: Deserialise {
    /// Create a new Json.
    ///
    /// Create a new Json instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::Json;
    ///
    /// let content = "1";
    /// let deserialiser: Json<'_, u8> = Json::new(&content);
    /// ```
    pub fn new(content: &'c str) -> Self {
        Self { 
            content,
            phantom: PhantomData
        }
    }
}

impl<'c, 'de, T> Deserialiser<'de, T> for Json<'c, T>
where
    T: Deserialise,
{
    /// Deserialise.
    ///
    /// Deserialise the current deserialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::{Deserialiser, Json, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialised: u8 = Json::new(content).deserialise()?;
    ///     Ok(())
    /// }
    /// ```
    fn deserialise(&self) -> Result<T, ParseError> {
        T::accept::<T>(self, self.content)
    }

    /// Visit U8.
    ///
    /// Visit a u8 node and parse it for deserialisation.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::{Deserialiser, Json, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialiser: Json<'_, u8> = Json::new(&content);
    ///     assert_eq!("1", deserialiser.visit_u8(content)?);
    ///     Ok(())
    /// }
    /// ```
    fn visit_u8(&'de self, content: &'de str) -> Result<&'de str, ParseError> {
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Json::new must create as per struct initialisation.
    ///
    /// The new method on Json must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn json_new() {
        let content = "1";
        let expected: Json<'_, u8> = Json { content: &content, phantom: PhantomData };
        let actual: Json<'_, u8> = Json::new(&content);

        assert_eq!(expected, actual);
    }

    /// Json::visit_u8 must correctly deserialise a u8.
    ///
    /// The visit_u8 method must correctly deserialise u8 values.
    #[test]
    fn json_visit_u8() {
        let content = "1";
        let json: Json<'_, u8> = Json::new(&content);

        assert_eq!("1", json.visit_u8(content).unwrap());
        assert_eq!(1_u8, json.deserialise().unwrap());
    }


    /// Json::visit_u8 must error on invalid u8.
    ///
    /// The visit_u8 method must correctly return the value, however, deserialise
    /// must correctly error on an invalid u8 token.
    #[test]
    fn json_visit_u8_invalid() {
        let content = "a";
        let json: Json<'_, u8> = Json::new(&content);

        assert_eq!("a", json.visit_u8(content).unwrap());
        assert_eq!(ParseError::new("Invalid value 'a' for type 'u8'."), json.deserialise().unwrap_err());
    }
}
