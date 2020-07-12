const CODE_POINT_MAX: u32 = 0x10FFFF;

pub(crate) fn is_code_point(v: u32) -> bool {
    v <= CODE_POINT_MAX
}

pub(crate) fn is_noncharacter(v: u32) -> bool {
    match v {
        0xFDD0..=0xFDEF
        | 0xFFFE..=0xFFFF
        | 0x1FFFE..=0x1FFFF
        | 0x2FFFE..=0x2FFFF
        | 0x3FFFE..=0x3FFFF
        | 0x4FFFE..=0x4FFFF
        | 0x5FFFE..=0x5FFFF
        | 0x6FFFE..=0x6FFFF
        | 0x7FFFE..=0x7FFFF
        | 0x8FFFE..=0x8FFFF
        | 0x9FFFE..=0x9FFFF
        | 0x10FFFE..=0x10FFFF => true,
        _ => false,
    }
}
