//! Regression tests for https://github.com/dtolnay/monostate/issues/31
//!
//! The fixed-width `MustBe` integer types must interoperate with deserializers
//! whose `deserialize_any` reports an integer through a `visit_u*` method other
//! than the one the target width uses. Self-describing formats disagree on which
//! width `deserialize_any` reports:
//!
//!  - `pythonize` narrows: the integer `3` is reported via `visit_u8`, which
//!    broke `MustBe!(3u64)`.
//!  - `serde_json` / `cbor4ii` widen: `3` is reported via `visit_u64`, which
//!    broke `MustBe!(3u8)`.
//!
//! Both are resolved by requesting the concrete serde type via the matching
//! `deserialize_u*` hint (which format-honoring deserializers respect), and by
//! relying on serde's default visitor forwarding (`visit_u8` -> `visit_u64`)
//! instead of hand-rolled rejections for deserializers that ignore the hint.

use monostate::MustBe;
use serde::de::value::{Error, U8Deserializer};
use serde::de::{Deserializer, IntoDeserializer, Visitor};
use serde::forward_to_deserialize_any;
use serde::Deserialize;

/// How a mock deserializer's `deserialize_any` reports a non-negative integer.
#[derive(Copy, Clone)]
enum Mode {
    /// Report through the *smallest* fitting `visit_u*`, like `pythonize`.
    Narrow,
    /// Report through `visit_u64`, like `serde_json` / `cbor4ii`.
    Widen,
}

/// A minimal deserializer holding one non-negative integer. Its concrete
/// `deserialize_u*` methods honor the requested width (as real self-describing
/// formats do); only `deserialize_any` follows `mode`.
struct IntDeserializer {
    value: u64,
    mode: Mode,
}

impl<'de> Deserializer<'de> for IntDeserializer {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.mode {
            Mode::Narrow => {
                if let Ok(v) = u8::try_from(self.value) {
                    visitor.visit_u8(v)
                } else if let Ok(v) = u16::try_from(self.value) {
                    visitor.visit_u16(v)
                } else if let Ok(v) = u32::try_from(self.value) {
                    visitor.visit_u32(v)
                } else {
                    visitor.visit_u64(self.value)
                }
            }
            Mode::Widen => visitor.visit_u64(self.value),
        }
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_u8(self.value as u8)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_u16(self.value as u16)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_u32(self.value as u32)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_u64(self.value)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map struct
        enum identifier ignored_any
    }
}

/// pythonize case: a narrowing but hint-honoring deserializer reports `3` as
/// `visit_u8` from `deserialize_any`, so `MustBe!(3u64)` must request `u64`.
#[test]
fn u64_from_narrowing_deserializer() {
    type Three = MustBe!(3u64);
    let de = IntDeserializer {
        value: 3,
        mode: Mode::Narrow,
    };
    Three::deserialize(de).unwrap();
}

/// cbor4ii case: a widening but hint-honoring deserializer reports `3` as
/// `visit_u64` from `deserialize_any`, so `MustBe!(3u8)` must request `u8`.
#[test]
fn u8_from_widening_deserializer() {
    type Three = MustBe!(3u8);
    let de = IntDeserializer {
        value: 3,
        mode: Mode::Widen,
    };
    Three::deserialize(de).unwrap();
}

/// Hint-ignoring case: serde's own `U8Deserializer` forwards every method to
/// `deserialize_any` and reports `visit_u8`. `MustBe!(3u64)` must accept it via
/// serde's default `visit_u8` -> `visit_u64` forwarding rather than rejecting.
#[test]
fn u64_from_hint_ignoring_u8_deserializer() {
    type Three = MustBe!(3u64);
    let de: U8Deserializer<Error> = 3u8.into_deserializer();
    Three::deserialize(de).unwrap();
}

/// A wrong value must still be rejected.
#[test]
fn wrong_value_is_rejected() {
    type Three = MustBe!(3u64);
    let de = IntDeserializer {
        value: 4,
        mode: Mode::Narrow,
    };
    assert!(Three::deserialize(de).is_err());
}

/// If `zarr_format` were `MustBe!(3)` (-> `MustBePosInt`) instead of
/// `MustBe!(3u64)`, it survives the untagged `ContentDeserializer` replay:
/// `MustBePosInt` is value-based (implements `visit_u64`) and has no reject
/// overrides, so the replayed `visit_u8(3)` forwards up to `visit_u64(3)`.
#[test]
fn posint_from_hint_ignoring_u8_deserializer() {
    type Three = MustBe!(3);
    let de: U8Deserializer<Error> = 3u8.into_deserializer();
    Three::deserialize(de).unwrap();
}
