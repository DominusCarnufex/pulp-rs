pub fn u8x2_to_u16(array : &[u8]) -> u16 {
    array[0] as u16 + array[1] as u16 * 0x100
}

pub fn u8x4_to_u32(array : &[u8]) -> u32 {
    let x : u32 = 0x100;
    array[0] as u32 + array[1] as u32 * x + array[2] as u32 * x.pow(2)
        + array[3] as u32 * x.pow(3)
}

pub fn u8x8_to_i64(array : &[u8]) -> i64 {
    let x : u64 = 0x100;
    (array[0] as u64 + array[1] as u64 * x + array[2] as u64 * x.pow(2)
        + array[3] as u64 * x.pow(3) + array[4] as u64 * x.pow(4)
        + array[5] as u64 * x.pow(5) + array[6] as u64 * x.pow(6)
        + array[7] as u64 * x.pow(7)) as i64
}
