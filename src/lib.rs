#![no_std]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![allow(
    clippy::borrow_as_ptr,
    clippy::builtin_type_shadow,
    clippy::expl_impl_clone_on_copy,
    clippy::missing_safety_doc,
    clippy::ptr_as_ptr
)]

#[doc(hidden)]
pub mod alphabet;
mod debug;
mod default;
mod eq;
mod hash;
mod ord;
mod partial_eq;
mod partial_ord;
mod serialize;
mod string;

pub use monostate_impl::MustBe;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeChar<const char: char>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBePosInt<const u128: u128>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeNegInt<const i128: i128>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeU8<const u8: u8>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeU16<const u16: u16>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeU32<const u32: u32>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeU64<const u64: u64>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeU128<const u128: u128>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeI8<const i8: i8>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeI16<const i16: i16>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeI32<const i32: i32>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeI64<const i64: i64>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeI128<const i128: i128>;

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct MustBeBool<const bool: bool>;

mod void {
    use core::marker::PhantomData;

    enum Void {}

    impl Copy for Void {}

    impl Clone for Void {
        fn clone(&self) -> Self {
            match *self {}
        }
    }

    pub struct MustBeStr<T>(PhantomData<T>, Void);

    impl<T> Copy for MustBeStr<T> {}

    impl<T> Clone for MustBeStr<T> {
        fn clone(&self) -> Self {
            *self
        }
    }
}

mod value {
    pub use super::MustBeStr::MustBeStr;
}

#[doc(hidden)]
pub enum MustBeStr<str> {
    __Phantom(void::MustBeStr<str>),
    MustBeStr,
}

impl<str> Copy for MustBeStr<str> {}

impl<str> Clone for MustBeStr<str> {
    fn clone(&self) -> Self {
        *self
    }
}

#[doc(hidden)]
pub use self::value::*;
