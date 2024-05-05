use std::ops::Range;

pub const COMMON_CHARS: [u16; 22] = [45, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 95, 1632, 1633, 1634, 1635, 1636, 1637, 1638, 1639, 1640, 1641];
pub const LATIN_UPPER: Range<u16> = 65..90;
pub const LATIN_LOWER: Range<u16> = 97..122;
pub const LATIN_ACCENTS: Range<u16> = 192..255;
