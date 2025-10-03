use core::fmt::{self, Octal};

impl<const V: u128> Octal for crate::MustBePosInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> Octal for crate::MustBeNegInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u8> Octal for crate::MustBeU8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u16> Octal for crate::MustBeU16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u32> Octal for crate::MustBeU32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u64> Octal for crate::MustBeU64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> Octal for crate::MustBeU128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i8> Octal for crate::MustBeI8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i16> Octal for crate::MustBeI16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i32> Octal for crate::MustBeI32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i64> Octal for crate::MustBeI64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> Octal for crate::MustBeI128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}
