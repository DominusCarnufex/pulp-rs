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
