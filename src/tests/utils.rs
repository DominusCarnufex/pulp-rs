use super::super::utils;

#[test]
fn u8x2_to_u16()    {
    let vec = vec![0x12, 0x42];
    assert_eq!(0x4212, utils::u8x2_to_u16(&vec));
}

#[test]
fn u8x4_to_u32()    {
    let vec = vec![0x12, 0x42, 0xee, 0xfa];
    assert_eq!(0xfaee4212, utils::u8x4_to_u32(&vec));
}

#[test]
fn u8x8_to_i64()    {
    let vec = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    assert_eq!(-1, utils::u8x8_to_i64(&vec));
}
