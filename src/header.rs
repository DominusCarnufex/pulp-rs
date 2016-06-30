use ::utils::u8x4_to_u32;

pub const HEADER_LENGTH : usize = 28;

pub struct Header   {
    pub vers_maj   : u8,
    pub vers_min   : u8,
    pub vers_patch : u8,
    pub timestamp  : u32,
    pub md5        : (u32, u32, u32, u32)
}

pub fn header(bytecode : &[u8]) -> Result<Header, String> {
    let magic = &bytecode[0..4];
    if *magic != [0x50, 0x55, 0x4c, 0x50]   {
        return Err("Le fichier n’est pas un bytecode PULP : \
                    nombre magique incorrect.".to_string());
    }

    let md5 = (u8x4_to_u32(&bytecode[12..16]),
               u8x4_to_u32(&bytecode[16..20]),
               u8x4_to_u32(&bytecode[20..24]),
               u8x4_to_u32(&bytecode[24..28]));

    Ok(Header   {
        vers_maj   : bytecode[4],
        vers_min   : bytecode[5],
        vers_patch : bytecode[6],
        timestamp  : u8x4_to_u32(&bytecode[8..12]),
        md5        : md5
    })
}

#[cfg(test)]
mod tests   {
    #[test]
    fn header() {
        let vec = vec![
            0x50, 0x55, 0x4c, 0x50,
            0x00, 0x01, 0x00,
            0x00,
            0x57, 0x6e, 0xbc, 0xfa,
            0x82, 0x71, 0x3a, 0x3b, 0x49, 0x41, 0x98, 0x7c,
            0x15, 0x55, 0x41, 0x1d, 0x9e, 0xe4, 0xb6, 0x7b
        ];

        let h = match super::header(&vec)   {
            Ok(a)  => a,
            Err(e) => panic!("{}", e)
        };

        let md5 = (0x3b3a7182, 0x7c984149, 0x1d415515, 0x7bb6e49e);

        assert_eq!(0,          h.vers_maj);
        assert_eq!(1,          h.vers_min);
        assert_eq!(0,          h.vers_patch);
        assert_eq!(0xfabc6e57, h.timestamp);
        assert_eq!(md5,        h.md5);
    }

    #[test]
    #[should_panic(expected = "nombre magique incorrect")]
    fn bad_magic()  {
        let vec = vec![0x50, 0x75, 0x4c, 0x50];
        match super::header(&vec)   {
            Ok(_)  => {},
            Err(e) => panic!("{}", e)
        }
    }
} // End of tests.
