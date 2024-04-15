//! [![github]](https://github.com/dtolnay/monostate)&ensp;[![crates-io]](https://crates.io/crates/monostate)&ensp;[![docs-rs]](https://docs.rs/monostate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This library implements a type macro for a zero-sized type that is Serde
//! deserializable only from one specific value.
//!
//! # Examples
//!
//! ```
//! use monostate::MustBe;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Example {
//!     kind: MustBe!("success"),
//!     code: MustBe!(200),
//! }
//! ```
//!
//! The above struct would deserialize from `{"kind":"success", "code":200}` in
//! JSON, but would fail the deserialization if "kind" or "code" were any other
//! value.
//!
//! This can sometimes be helpful in processing untagged enums in which the
//! variant identification is more convoluted than what is handled by Serde's
//! externally tagged and internally tagged representations, for example because
//! the variant tag has an inconsistent type or key.
//!
//! ```
//! use monostate::MustBe;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! #[serde(untagged)]
//! pub enum ApiResponse {
//!     Success {
//!         success: MustBe!(true),
//!     },
//!     Error {
//!         kind: MustBe!("error"),
//!         message: String,
//!     },
//! }
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/monostate/0.1.12")]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![allow(
    clippy::borrow_as_ptr,
    clippy::builtin_type_shadow,
    clippy::cast_lossless,
    clippy::cast_sign_loss,
    clippy::expl_impl_clone_on_copy,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::ptr_as_ptr,
    clippy::uninhabited_references,
    clippy::uninlined_format_args
)]

#[doc(hidden)]
pub mod alphabet;
mod debug;
mod default;
mod deserialize;
mod eq;
mod format;
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
            *self
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
    #[doc(hidden)]
    pub use super::MustBeStr::MustBeStr;
}

// Equivalent to `pub struct MustBeStr<const str: &'static str>;` but using
// the type encoding described in impl/src/lib.rs to avoid depending on
// #![feature(adt_const_params)] for now.
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
