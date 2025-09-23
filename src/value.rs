use crate::string::{ConstStr, Sealed};
use core::primitive;

impl<const char: char> crate::MustBeChar<{ char }> {
    pub const VALUE: char = char;
}

impl<const u8: u8> crate::MustBeU8<{ u8 }> {
    pub const VALUE: u8 = u8;
}

impl<const u16: u16> crate::MustBeU16<{ u16 }> {
    pub const VALUE: u16 = u16;
}

impl<const u32: u32> crate::MustBeU32<{ u32 }> {
    pub const VALUE: u32 = u32;
}

impl<const u64: u64> crate::MustBeU64<{ u64 }> {
    pub const VALUE: u64 = u64;
}

impl<const u128: u128> crate::MustBeU128<{ u128 }> {
    pub const VALUE: u128 = u128;
}

impl<const i8: i8> crate::MustBeI8<{ i8 }> {
    pub const VALUE: i8 = i8;
}

impl<const i16: i16> crate::MustBeI16<{ i16 }> {
    pub const VALUE: i16 = i16;
}

impl<const i32: i32> crate::MustBeI32<{ i32 }> {
    pub const VALUE: i32 = i32;
}

impl<const i64: i64> crate::MustBeI64<{ i64 }> {
    pub const VALUE: i64 = i64;
}

impl<const i128: i128> crate::MustBeI128<{ i128 }> {
    pub const VALUE: i128 = i128;
}

impl<const bool: bool> crate::MustBeBool<{ bool }> {
    pub const VALUE: bool = bool;
}

impl<str: ConstStr> crate::MustBeStr<str> {
    pub const VALUE: &'static primitive::str = const { <str as Sealed>::VALUE };
}
