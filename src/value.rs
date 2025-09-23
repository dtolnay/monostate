use crate::string::ConstStr;

pub trait MustBe: Sealed {
    type Type;
    const VALUE: Self::Type;
}

pub trait Sealed {}

impl<const V: char> crate::MustBeChar<V> {
    pub const VALUE: char = V;
}

impl<const V: u8> crate::MustBeU8<V> {
    pub const VALUE: u8 = V;
}

impl<const V: u16> crate::MustBeU16<V> {
    pub const VALUE: u16 = V;
}

impl<const V: u32> crate::MustBeU32<V> {
    pub const VALUE: u32 = V;
}

impl<const V: u64> crate::MustBeU64<V> {
    pub const VALUE: u64 = V;
}

impl<const V: u128> crate::MustBeU128<V> {
    pub const VALUE: u128 = V;
}

impl<const V: i8> crate::MustBeI8<V> {
    pub const VALUE: i8 = V;
}

impl<const V: i16> crate::MustBeI16<V> {
    pub const VALUE: i16 = V;
}

impl<const V: i32> crate::MustBeI32<V> {
    pub const VALUE: i32 = V;
}

impl<const V: i64> crate::MustBeI64<V> {
    pub const VALUE: i64 = V;
}

impl<const V: i128> crate::MustBeI128<V> {
    pub const VALUE: i128 = V;
}

impl<const V: bool> crate::MustBeBool<V> {
    pub const VALUE: bool = V;
}

impl<V: ConstStr> crate::MustBeStr<V> {
    pub const VALUE: &'static str = const { <V as crate::string::Sealed>::VALUE };
}
