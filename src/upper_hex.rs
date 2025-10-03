use core::fmt::{self, UpperHex};

impl<const V: u128> UpperHex for crate::MustBePosInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> UpperHex for crate::MustBeNegInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u8> UpperHex for crate::MustBeU8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u16> UpperHex for crate::MustBeU16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u32> UpperHex for crate::MustBeU32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u64> UpperHex for crate::MustBeU64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> UpperHex for crate::MustBeU128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i8> UpperHex for crate::MustBeI8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i16> UpperHex for crate::MustBeI16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i32> UpperHex for crate::MustBeI32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i64> UpperHex for crate::MustBeI64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> UpperHex for crate::MustBeI128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}
