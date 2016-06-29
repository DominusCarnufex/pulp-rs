use std::error::Error; // Nécessaire pour e.description().
use ::utils::*;
use ::header::HEADER_LENGTH;

#[derive(Clone, Debug, PartialEq)]
pub enum Segment    {
    Code {
        symbol_table : Vec<Symbol>,
        const_table  : Vec<Const>,
        code         : Vec<Opcode>
    },
}

pub fn segments(bytecode : &[u8]) -> Result<Vec<Segment>, String>   {
    let mut vec    = Vec::new();
    let mut offset = 0;

    while offset < bytecode.len() - 1   {
        let size  = u8x4_to_u32(&bytecode[offset .. offset + 4]) as usize;
        if offset + size > bytecode.len()   {
            return Err(format!("Segment invalide à l’offset 0x{:x} : \
                    taille incorrecte.", offset + HEADER_LENGTH));
        }

        let stype = bytecode[offset + 4];
        let segment = match stype   {
            0x01 => code_segment(
                        &bytecode[offset + 8 .. offset + size],
                        offset
                    ),
            _    => Err(format!("Segment invalide à l’offset 0x{:x} : \
                    type inconnu.", offset + HEADER_LENGTH))
        };

        match segment   {
            Ok(a)  => vec.push(a),
            Err(e) => return Err(e)
        }

        offset += size;
    } // End of loop.

    Ok(vec)
} // End of segments() function.

pub type Symbol = String;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Const  {
    Int(i64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    NOp,
    Pop,
    Rot2,
    Rot3,
    DupTop,
    Push(usize),
    PushNewEnv,
    PopEnv,
    Let(usize),
    Store(usize),
    Load(usize),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    BitOr,
    BitAnd,
    BitXor,
    LShift,
    RShift,
    UMinus,
}

fn code_segment(bytecode : &[u8], overhead : usize)
    -> Result<Segment, String>
{
    if bytecode.len() < 12   {
        return Err(format!("Segment invalide à l’offset 0x{:x} : \
                    trop petit pour être valide.", overhead));
    }

    let mut offset = 0;

    /***** SYMBOL TABLE *****/
    let symbol_table_size =
        u8x2_to_u16(&bytecode[offset .. offset + 2]) as usize;

    if bytecode.len() < offset + symbol_table_size || symbol_table_size < 4 {
        return Err(format!("Table des symboles invalide à l’offset 0x{:x} : \
                    trop petite pour être valide.", offset + overhead));
    }

    let symbol_count =
        u8x2_to_u16(&bytecode[offset + 2 .. offset + 4]) as usize;
    offset += 4;

    let mut symbols  = Vec::with_capacity(symbol_count);
    for _i in 0..symbol_count   {
        let     size = bytecode[offset] as usize;
        let mut vec  = Vec::with_capacity(size);
        for u in &bytecode[offset + 1 .. offset + size + 1] {
            vec.push(*u);
        }

        let symbol = match String::from_utf8(vec)   {
            Ok(a)  => a,
            Err(e) => return Err(
                        format!("Symbole invalide à l’offset 0x{:x} : \
                            UTF-8 invalide ({}).", offset + overhead,
                            e.description())
                      )
        };

        symbols.push(symbol);
        offset += size + 1;
    } // End of loop.

    /***** CONSTANT TABLE *****/
    let const_table_size =
        u8x2_to_u16(&bytecode[offset .. offset + 2]) as usize;

    if bytecode.len() < offset + const_table_size || const_table_size < 4   {
        return Err(format!("Table des constantes invalide à l’offset 0x{:x} : \
                    trop petite pour être valide.", offset + overhead));
    }

    let const_count =
        u8x2_to_u16(&bytecode[offset + 2 .. offset + 4]) as usize;
    offset += 4;

    let mut consts  = Vec::with_capacity(const_count);
    for _i in 0..const_count    {
        let constant = match bytecode[offset]   {
            0x01 => Const::Int(
                        u8x8_to_i64(&bytecode[offset + 1 .. offset + 9])
                    ),
            _    => return Err(
                        format!("Constante invalide à l’offset 0x{:x} : \
                            type inconnu.", offset + overhead)
                    )
        };

        consts.push(constant);

        offset += match bytecode[offset]    {
            0x01 => 9,
            _    => 0
        };
    } // End of loop.

    /***** OPCODE LIST *****/
    let opcode_list_size =
        u8x2_to_u16(&bytecode[offset .. offset + 2]) as usize;

    if bytecode.len() < offset + opcode_list_size || opcode_list_size < 4   {
        return Err(format!("Section de code invalide à l’offset 0x{:x} : \
                    trop petite pour être valide.", offset + overhead));
    }

    let opcode_count =
        u8x2_to_u16(&bytecode[offset + 2 .. offset + 4]) as usize;
    offset += 4;

    let mut opcodes  = Vec::with_capacity(opcode_count);
    for _i in 0..opcode_count   {
        let opcode = match bytecode[offset] {
            0x00 => Opcode::NOp,
            0x01 => Opcode::Pop,
            0x02 => Opcode::Rot2,
            0x03 => Opcode::Rot3,
            0x04 => Opcode::DupTop,
            0x0a => {
                offset += 2;
                let arg = u8x2_to_u16(
                    &bytecode[offset - 1 .. offset + 1]
                ) as usize;
                Opcode::Push(arg)
            },
            0x20 => Opcode::PushNewEnv,
            0x21 => Opcode::PopEnv,
            0x25 => {
                offset += 2;
                let arg = u8x2_to_u16(
                    &bytecode[offset - 1 .. offset + 1]
                ) as usize;
                Opcode::Let(arg)
            },
            0x27 => {
                offset += 2;
                let arg = u8x2_to_u16(
                    &bytecode[offset - 1 .. offset + 1]
                ) as usize;
                Opcode::Store(arg)
            },
            0x29 => {
                offset += 2;
                let arg = u8x2_to_u16(
                    &bytecode[offset - 1 .. offset + 1]
                ) as usize;
                Opcode::Load(arg)
            },
            0x30 => Opcode::Add,
            0x31 => Opcode::Sub,
            0x32 => Opcode::Mul,
            0x33 => Opcode::Div,
            0x34 => Opcode::Pow,
            0x35 => Opcode::Mod,
            0x36 => Opcode::BitOr,
            0x37 => Opcode::BitAnd,
            0x38 => Opcode::BitXor,
            0x39 => Opcode::LShift,
            0x3a => Opcode::RShift,
            0x40 => Opcode::UMinus,
            _    => return Err(
                        format!("Opcode invalide à l’offset 0x{:x} : \
                            type inconnu.", offset + overhead)
                    )
        }; // End of match.

        opcodes.push(opcode);
        offset += 1;
    } // End of loop.

    Ok(Segment::Code    {
        symbol_table : symbols,
        const_table  : consts,
        code         : opcodes,
    })
} // End of code_segment() function.
