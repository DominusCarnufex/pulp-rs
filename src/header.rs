use ::utils::u8x4_to_u32;

pub const HEADER_LENGTH : usize = 28;

pub struct Header   {
    pub vers_maj   : u8,
    pub vers_min   : u8,
    pub vers_patch : u8,
    pub timestamp  : u32,
//    md5        : String
}

pub fn header(bytecode : &[u8]) -> Result<Header, String> {
    let magic = &bytecode[0..4];
    if *magic != [0x50, 0x55, 0x4c, 0x50]   {
        return Err("Le fichier n’est pas un bytecode PULP : \
                    nombre magique incorrect.".to_string());
    }

    Ok(Header   {
        vers_maj   : bytecode[4],
        vers_min   : bytecode[5],
        vers_patch : bytecode[6],
        timestamp  : u8x4_to_u32(&bytecode[8..12]),
//        md5        : String
    })
}

#[cfg(test)]
mod tests   {
    #[test]
    fn header() {
        let vec = vec![
            0x50, 0x55, 0x4c, 0x50, //    ("PULP") 
            0x00, 0x01, 0x00,       //    (0.1.0)
            0x00,                   //    (padding)
            0x57, 0x6e, 0xbc, 0xfa, //    (2016-06-25 19:18:50)
            0x17, 0x6d, 0x7d, 0x7f, 0xde, 0x3f, 0x69, 0x00, // (md5)
            0xc8, 0x7f, 0x64, 0x77, 0x11, 0xc3, 0xab, 0xb2,
        ];

        let h = match super::header(&vec)   {
            Ok(a)  => a,
            Err(e) => panic!("{}", e)
        };

        assert_eq!(0,          h.vers_maj);
        assert_eq!(1,          h.vers_min);
        assert_eq!(0,          h.vers_patch);
        assert_eq!(0xfabc6e57, h.timestamp);
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
