//! Serialize module which houses the Serialize and Serializer traits used to
//! handle the serialization process. Also houses the implementation of
//! Serialize on supported core items.

mod json;
pub use json::Json;

/// Trait to implement on serializable items. Defines how the item is
/// serialized.
pub trait Serialize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer;
}

impl Serialize for () {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_unit()
    }
}

impl Serialize for bool {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_bool(self)
    }
}

impl Serialize for char {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_char(self)
    }
}

impl Serialize for f32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_f32(self)
    }
}

impl Serialize for f64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_f64(self)
    }
}

impl Serialize for i8 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i8(self)
    }
}

impl Serialize for i16 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i16(self)
    }
}

impl Serialize for i32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i32(self)
    }
}

impl Serialize for i64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i64(self)
    }
}

impl Serialize for i128 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i128(self)
    }
}

impl Serialize for isize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_isize(self)
    }
}

impl Serialize for str {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_str(self)
    }
}

impl Serialize for String {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_string(self)
    }
}

impl Serialize for u8 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u8(self)
    }
}

impl Serialize for u16 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u16(self)
    }
}

impl Serialize for u32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u32(self)
    }
}

impl Serialize for u64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u64(self)
    }
}

impl Serialize for u128 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u128(self)
    }
}

impl Serialize for usize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_usize(self)
    }
}

/// Trait to implement on an item that conducts the serialization, and defines
/// how data is serialized. Interaction with this should be done using the
/// serialize method, which in turn calls the required visit methods to
/// convert into the output type.
pub trait Serializer {
    /// The output type for this Serializer.
    type Output;

    /// Serialize the input into the required output type.
    fn serialize<S>(&self, input: &S) -> Self::Output
    where
        S: Serialize + ?Sized;

    /// Visit and serialize a bool type.
    fn visit_bool(&self, input: &bool) -> Self::Output;

    /// Visit and serialize a char type.
    fn visit_char(&self, input: &char) -> Self::Output;

    /// Visit and serialize a f32 type.
    fn visit_f32(&self, input: &f32) -> Self::Output;

    /// Visit and serialize a f64 type.
    fn visit_f64(&self, input: &f64) -> Self::Output;

    /// Visit and serialize an i8 type.
    fn visit_i8(&self, input: &i8) -> Self::Output;

    /// Visit and serialize an i16 type.
    fn visit_i16(&self, input: &i16) -> Self::Output;

    /// Visit and serialize an i32 type.
    fn visit_i32(&self, input: &i32) -> Self::Output;

    /// Visit and serialize an i64 type.
    fn visit_i64(&self, input: &i64) -> Self::Output;

    /// Visit and serialize an i128 type.
    fn visit_i128(&self, input: &i128) -> Self::Output;

    /// Visit and serialize an isize type.
    fn visit_isize(&self, input: &isize) -> Self::Output;

    /// Visit and serialize a str type.
    fn visit_str(&self, input: &str) -> Self::Output;

    /// Visit and serialize a String type.
    fn visit_string(&self, input: &String) -> Self::Output;

    /// Visit and serialize an u8 type.
    fn visit_u8(&self, input: &u8) -> Self::Output;

    /// Visit and serialize an u16 type.
    fn visit_u16(&self, input: &u16) -> Self::Output;

    /// Visit and serialize an u32 type.
    fn visit_u32(&self, input: &u32) -> Self::Output;

    /// Visit and serialize an u64 type.
    fn visit_u64(&self, input: &u64) -> Self::Output;

    /// Visit and serialize an u128 type.
    fn visit_u128(&self, input: &u128) -> Self::Output;

    /// Visit and serialize a unit type.
    fn visit_unit(&self) -> Self::Output;

    /// Visit and serialize an usize type.
    fn visit_usize(&self, input: &usize) -> Self::Output;
}
