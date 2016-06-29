pub fn u8x2_to_u16(array : &[u8]) -> u16 {
    array[0] as u16 + array[1] as u16 * 256
}

pub fn u8x4_to_u32(array : &[u8]) -> u32 {
    let x : u32 = 256;
    array[0] as u32 + array[1] as u32 * x + array[2] as u32 * x.pow(2)
        + array[3] as u32 * x.pow(3)
}

pub fn u8x8_to_i64(array : &[u8]) -> i64 {
    let x : u64 = 256;
    (array[0] as u64 + array[1] as u64 * x + array[2] as u64 * x.pow(2)
        + array[3] as u64 * x.pow(3) + array[4] as u64 * x.pow(4)
        + array[5] as u64 * x.pow(5) + array[6] as u64 * x.pow(6)
        + array[7] as u64 * x.pow(7)) as i64
}

#[cfg(test)]
mod tests   {
    #[test]
    fn u8x2_to_u16()    {
        let vec = vec![0x12, 0x42];
        assert_eq!(0x4212, super::u8x2_to_u16(&vec));
    }

    #[test]
    fn u8x4_to_u32()    {
        let vec = vec![0x12, 0x42, 0xee, 0xfa];
        assert_eq!(0xfaee4212, super::u8x4_to_u32(&vec));
    }

    #[test]
    fn u8x8_to_i64()    {
        let vec = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        assert_eq!(-1, super::u8x8_to_i64(&vec));
    }
} // End of tests.
