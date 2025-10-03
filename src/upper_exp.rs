use core::fmt::{self, UpperExp};

impl<const V: u128> UpperExp for crate::MustBePosInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> UpperExp for crate::MustBeNegInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u8> UpperExp for crate::MustBeU8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u16> UpperExp for crate::MustBeU16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u32> UpperExp for crate::MustBeU32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u64> UpperExp for crate::MustBeU64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> UpperExp for crate::MustBeU128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i8> UpperExp for crate::MustBeI8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i16> UpperExp for crate::MustBeI16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i32> UpperExp for crate::MustBeI32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i64> UpperExp for crate::MustBeI64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> UpperExp for crate::MustBeI128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}
