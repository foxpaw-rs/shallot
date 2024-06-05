//! Deserialize module which houses the Deserialize and Deserializer traits
//! used to handle the deserialization process. Also houses the implementation
//! of Deserialize on supported core items.

mod json;

use crate::error::Result;
pub use json::Json;

/// Trait to implement on deserializable items. Defines how the item is
/// deserialized.
pub trait Deserialize {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and
    /// is uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
        Self: Sized;
}

impl Deserialize for () {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_unit(input)
    }
}

impl<A> Deserialize for (A,)
where
    A: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_1(input)
    }
}

impl<A, B> Deserialize for (A, B)
where
    A: Deserialize,
    B: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_2(input)
    }
}

impl<A, B, C> Deserialize for (A, B, C)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_3(input)
    }
}

impl<A, B, C, D> Deserialize for (A, B, C, D)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_4(input)
    }
}

impl<A, B, C, D, E> Deserialize for (A, B, C, D, E)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_5(input)
    }
}

impl<A, B, C, D, E, F> Deserialize for (A, B, C, D, E, F)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_6(input)
    }
}

impl<A, B, C, D, E, F, G> Deserialize for (A, B, C, D, E, F, G)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_7(input)
    }
}

impl<A, B, C, D, E, F, G, H> Deserialize for (A, B, C, D, E, F, G, H)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
    H: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_8(input)
    }
}

impl<A, B, C, D, E, F, G, H, I> Deserialize for (A, B, C, D, E, F, G, H, I)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
    H: Deserialize,
    I: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_9(input)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> Deserialize for (A, B, C, D, E, F, G, H, I, J)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
    H: Deserialize,
    I: Deserialize,
    J: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_10(input)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> Deserialize for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
    H: Deserialize,
    I: Deserialize,
    J: Deserialize,
    K: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_11(input)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Deserialize for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: Deserialize,
    B: Deserialize,
    C: Deserialize,
    D: Deserialize,
    E: Deserialize,
    F: Deserialize,
    G: Deserialize,
    H: Deserialize,
    I: Deserialize,
    J: Deserialize,
    K: Deserialize,
    L: Deserialize,
{
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_tuple_12(input)
    }
}

impl Deserialize for bool {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_bool(input)
    }
}

impl Deserialize for char {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_char(input)
    }
}

impl Deserialize for f32 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_f32(input)
    }
}

impl Deserialize for f64 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_f64(input)
    }
}

impl Deserialize for i8 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_i8(input)
    }
}

impl Deserialize for i16 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_i16(input)
    }
}

impl Deserialize for i32 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_i32(input)
    }
}

impl Deserialize for i64 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_i64(input)
    }
}

impl Deserialize for i128 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_i128(input)
    }
}

impl Deserialize for isize {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_isize(input)
    }
}

impl Deserialize for String {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_string(input)
    }
}

impl Deserialize for u8 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_u8(input)
    }
}

impl Deserialize for u16 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_u16(input)
    }
}

impl Deserialize for u32 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_u32(input)
    }
}

impl Deserialize for u64 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_u64(input)
    }
}

impl Deserialize for u128 {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_u128(input)
    }
}

impl Deserialize for usize {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_usize(input)
    }
}

/// Trait to implement on an item that conducts the deserialization, and
/// defines how data is deserialized. Interaction with this should be done
/// using the deserialize method, which in turn calls the required visit
/// methods to convert from the input type into the specified output type.
pub trait Deserializer {
    /// The input type for this Deserializer.
    type Input;

    /// Deserialize the input into the required output type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn deserialize<S>(&self, input: &Self::Input) -> Result<S>
    where
        S: Deserialize;

    /// Visit and deserialize a bool type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_bool(&self, input: &Self::Input) -> Result<bool>;

    /// Visit and deserialize a char type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_char(&self, input: &Self::Input) -> Result<char>;

    /// Visit and deserialize an f32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_f32(&self, input: &Self::Input) -> Result<f32>;

    /// Visit and deserialize an f64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_f64(&self, input: &Self::Input) -> Result<f64>;

    /// Visit and deserialize an i8 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i8(&self, input: &Self::Input) -> Result<i8>;

    /// Visit and deserialize an i16 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i16(&self, input: &Self::Input) -> Result<i16>;

    /// Visit and deserialize an i32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i32(&self, input: &Self::Input) -> Result<i32>;

    /// Visit and deserialize an i64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i64(&self, input: &Self::Input) -> Result<i64>;

    /// Visit and deserialize an i128 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i128(&self, input: &Self::Input) -> Result<i128>;

    /// Visit and deserialize an isize type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_isize(&self, input: &Self::Input) -> Result<isize>;

    /// Visit and deserialize a String type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_string(&self, input: &Self::Input) -> Result<String>;

    /// Visit and deserialize a tuple type of size 1.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_1<A>(&self, input: &Self::Input) -> Result<(A,)>
    where
        A: Deserialize;

    /// Visit and deserialize a tuple type of size 2.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_2<A, B>(&self, input: &Self::Input) -> Result<(A, B)>
    where
        A: Deserialize,
        B: Deserialize;

    /// Visit and deserialize a tuple type of size 3.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_3<A, B, C>(&self, input: &Self::Input) -> Result<(A, B, C)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize;

    /// Visit and deserialize a tuple type of size 4.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_4<A, B, C, D>(&self, input: &Self::Input) -> Result<(A, B, C, D)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize;

    /// Visit and deserialize a tuple type of size 5.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_5<A, B, C, D, E>(&self, input: &Self::Input) -> Result<(A, B, C, D, E)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize;

    /// Visit and deserialize a tuple type of size 6.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_6<A, B, C, D, E, F>(&self, input: &Self::Input) -> Result<(A, B, C, D, E, F)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize;

    /// Visit and deserialize a tuple type of size 7.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_7<A, B, C, D, E, F, G>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize;

    /// Visit and deserialize a tuple type of size 8.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_8<A, B, C, D, E, F, G, H>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize;

    /// Visit and deserialize a tuple type of size 9.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_9<A, B, C, D, E, F, G, H, I>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize;

    /// Visit and deserialize a tuple type of size 10.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_10<A, B, C, D, E, F, G, H, I, J>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize;

    /// Visit and deserialize a tuple type of size 11.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_11<A, B, C, D, E, F, G, H, I, J, K>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J, K)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize,
        K: Deserialize;

    /// Visit and deserialize a tuple type of size 12.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_12<A, B, C, D, E, F, G, H, I, J, K, L>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize,
        K: Deserialize,
        L: Deserialize;

    /// Visit and deserialize a u8 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_u8(&self, input: &Self::Input) -> Result<u8>;

    /// Visit and deserialize a u16 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_u16(&self, input: &Self::Input) -> Result<u16>;

    /// Visit and deserialize a u32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_u32(&self, input: &Self::Input) -> Result<u32>;

    /// Visit and deserialize a u64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_u64(&self, input: &Self::Input) -> Result<u64>;

    /// Visit and deserialize a u128 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_u128(&self, input: &Self::Input) -> Result<u128>;

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_unit(&self, input: &Self::Input) -> Result<()>;

    /// Visit and deserialize a usize type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_usize(&self, input: &Self::Input) -> Result<usize>;
}
