use crate::string::ConstStr;
use core::fmt::{self, Display};

impl<const V: char> Display for crate::MustBeChar<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> Display for crate::MustBePosInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> Display for crate::MustBeNegInt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u8> Display for crate::MustBeU8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u16> Display for crate::MustBeU16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u32> Display for crate::MustBeU32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u64> Display for crate::MustBeU64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: u128> Display for crate::MustBeU128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i8> Display for crate::MustBeI8<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i16> Display for crate::MustBeI16<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i32> Display for crate::MustBeI32<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i64> Display for crate::MustBeI64<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: i128> Display for crate::MustBeI128<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<const V: bool> Display for crate::MustBeBool<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V.fmt(f)
    }
}

impl<V: ConstStr> Display for crate::MustBeStr<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        V::VALUE.fmt(f)
    }
}
