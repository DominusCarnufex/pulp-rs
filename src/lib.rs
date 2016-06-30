#![allow(dead_code)]
#![feature(stmt_expr_attributes)]

mod utils;

mod md5;
use md5::*;

mod header;
use header::*;

#[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
mod v0_1_0;

pub fn run(bytecode : &[u8]) -> Result<(), String>  {
    if bytecode.len() < HEADER_LENGTH   {
        return Err("Le fichier n’est pas un bytecode PULP : \
                    taille insuffisante.".to_string());
    }

    let header = match header(&bytecode)    {
        Ok(a)  => a,
        Err(e) => return Err(e)
    };

    if header.md5 != md5(&bytecode[HEADER_LENGTH..])    {
        return Err("Fichier corrompu : condensat MD5 incorrect.".to_string());
    }

    if header.vers_maj == 0 && header.vers_min == 1 &&
        header.vers_patch == 0
    {
        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        let segments = match v0_1_0::segments(&bytecode[HEADER_LENGTH..])   {
            Ok(a)  => a,
            Err(e) => return Err(e)
        };

        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        for s in segments   {
            match s.execute((Vec::new(), vec![v0_1_0::Environment::new()])) {
                Ok((mut stack, _)) => {
                    if stack.len() > 0  {
                        let v0_1_0::Const::Int(ts) = stack.pop().unwrap();
                        println!("Sommet de la pile : {}.", ts);
                    } else {
                        println!("Pile vide.");
                    }
                },
                Err(e)             => return Err(e)
            }
        }
    } // End of v0.1.0.

    Ok(())
} // End of run() function.

#[cfg(test)]
mod tests   {
    #[test]
    #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
    fn run_v0_1_0() {
        let vec = vec![
            0x50, 0x55, 0x4c, 0x50,
            0x00, 0x01, 0x00,
            0x00,
            0x57, 0x6e, 0xbc, 0xfa,
            0xc3, 0xe7, 0x2d, 0xcb, 0x23, 0xc5, 0x15, 0x20,
            0xae, 0x19, 0x62, 0x64, 0x5b, 0x9a, 0x8a, 0x9d,
            0x2d, 0x00, 0x00, 0x00,
            0x01,
            0x00, 0x00, 0x00,
            0x04, 0x00, 0x00, 0x00,
            0x16, 0x00, 0x02, 0x00,
              0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
              0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
            0x0b, 0x00, 0x03, 0x00,
              0x0a, 0x00, 0x00,
              0x0a, 0x01, 0x00,
              0x30
        ];

        match super::run(&vec)  {
            Ok(_)  => {},
            Err(e) => panic!("{}", e)
        };
    }

    #[test]
    #[should_panic(expected = "taille insuffisante")]
    fn too_small()  {
        let vec = vec![0x50, 0x75, 0x4c, 0x50];
        match super::run(&vec)  {
            Ok(_)  => {},
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    #[should_panic(expected = "condensat MD5 incorrect")]
    fn bad_md5()    {
        let vec = vec![
            0x50, 0x55, 0x4c, 0x50,
            0x00, 0x01, 0x00,
            0x00,
            0x57, 0x6e, 0xbc, 0xfa,
            0x17, 0x6d, 0x7d, 0x7f, 0xde, 0x3f, 0x69, 0x00,
            0xc8, 0x7f, 0x64, 0x77, 0x11, 0xc3, 0xab, 0xb2,
            0x2d, 0x00, 0x00, 0x00,
            0x01,
            0x00, 0x00, 0x00,
            0x04, 0x00, 0x00, 0x00,
            0x16, 0x00, 0x02, 0x00,
              0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
              0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
            0x0b, 0x00, 0x03, 0x00,
              0x0a, 0x00, 0x00,
              0x0a, 0x01, 0x00,
              0x30
        ];
        match super::run(&vec)  {
            Ok(_)  => {},
            Err(e) => panic!("{}", e)
        }
    }
} // End of tests.
