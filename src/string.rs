use crate::alphabet;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;

pub unsafe trait RetrieveString {
    // SAFETY: Must contain no padding bytes. Must have alignment of 1.
    type Type: 'static;
    // SAFETY: Contents viewed as bytes must be a valid UTF-8 encoding.
    const BYTES: Self::Type;
}

unsafe impl<const CH: char> RetrieveString for alphabet::char<CH> {
    type Type = u8;
    const BYTES: Self::Type = CH as u8;
}

unsafe impl<const CH: char> RetrieveString for alphabet::two::char<CH> {
    type Type = [u8; 2];
    const BYTES: Self::Type = [
        (CH as u32 >> 6 & 0x1F) as u8 | TAG_TWO_B,
        (CH as u32 & 0x3F) as u8 | TAG_CONT,
    ];
}

unsafe impl<const CH: char> RetrieveString for alphabet::three::char<CH> {
    type Type = [u8; 3];
    const BYTES: Self::Type = [
        (CH as u32 >> 12 & 0x0F) as u8 | TAG_THREE_B,
        (CH as u32 >> 6 & 0x3F) as u8 | TAG_CONT,
        (CH as u32 & 0x3F) as u8 | TAG_CONT,
    ];
}

unsafe impl<const CH: char> RetrieveString for alphabet::four::char<CH> {
    type Type = [u8; 4];
    const BYTES: Self::Type = [
        (CH as u32 >> 18 & 0x07) as u8 | TAG_FOUR_B,
        (CH as u32 >> 12 & 0x3F) as u8 | TAG_CONT,
        (CH as u32 >> 6 & 0x3F) as u8 | TAG_CONT,
        (CH as u32 & 0x3F) as u8 | TAG_CONT,
    ];
}

unsafe impl RetrieveString for () {
    type Type = ();
    const BYTES: Self::Type = ();
}

#[repr(C)]
pub struct Concat2<A, B>(A, B);

unsafe impl<A, B> RetrieveString for (A, B)
where
    A: RetrieveString,
    B: RetrieveString,
{
    type Type = Concat2<A::Type, B::Type>;
    const BYTES: Self::Type = Concat2(A::BYTES, B::BYTES);
}

#[repr(C)]
pub struct Concat3<A, B, C>(A, B, C);

unsafe impl<A, B, C> RetrieveString for (A, B, C)
where
    A: RetrieveString,
    B: RetrieveString,
    C: RetrieveString,
{
    type Type = Concat3<A::Type, B::Type, C::Type>;
    const BYTES: Self::Type = Concat3(A::BYTES, B::BYTES, C::BYTES);
}

#[repr(C)]
pub struct Concat4<A, B, C, D>(A, B, C, D);

unsafe impl<A, B, C, D> RetrieveString for (A, B, C, D)
where
    A: RetrieveString,
    B: RetrieveString,
    C: RetrieveString,
    D: RetrieveString,
{
    type Type = Concat4<A::Type, B::Type, C::Type, D::Type>;
    const BYTES: Self::Type = Concat4(A::BYTES, B::BYTES, C::BYTES, D::BYTES);
}

#[repr(C)]
pub struct Concat5<A, B, C, D, E>(A, B, C, D, E);

unsafe impl<A, B, C, D, E> RetrieveString for (A, B, C, D, E)
where
    A: RetrieveString,
    B: RetrieveString,
    C: RetrieveString,
    D: RetrieveString,
    E: RetrieveString,
{
    type Type = Concat5<A::Type, B::Type, C::Type, D::Type, E::Type>;
    const BYTES: Self::Type = Concat5(A::BYTES, B::BYTES, C::BYTES, D::BYTES, E::BYTES);
}

#[repr(C)]
pub struct Concat6<A, B, C, D, E, F>(A, B, C, D, E, F);

unsafe impl<A, B, C, D, E, F> RetrieveString for (A, B, C, D, E, F)
where
    A: RetrieveString,
    B: RetrieveString,
    C: RetrieveString,
    D: RetrieveString,
    E: RetrieveString,
    F: RetrieveString,
{
    type Type = Concat6<A::Type, B::Type, C::Type, D::Type, E::Type, F::Type>;
    const BYTES: Self::Type = Concat6(A::BYTES, B::BYTES, C::BYTES, D::BYTES, E::BYTES, F::BYTES);
}
