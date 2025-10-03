use core::fmt::{self, LowerExp};

impl<const V: u128> LowerExp for crate::MustBePosInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> LowerExp for crate::MustBeNegInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u8> LowerExp for crate::MustBeU8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u16> LowerExp for crate::MustBeU16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u32> LowerExp for crate::MustBeU32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u64> LowerExp for crate::MustBeU64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> LowerExp for crate::MustBeU128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i8> LowerExp for crate::MustBeI8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i16> LowerExp for crate::MustBeI16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i32> LowerExp for crate::MustBeI32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i64> LowerExp for crate::MustBeI64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> LowerExp for crate::MustBeI128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}
