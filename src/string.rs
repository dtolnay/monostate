use crate::alphabet;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;

unsafe trait RetrieveString {
    // SAFETY: Must contain no padding bytes. Must have alignment of 1.
    type Type;
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
