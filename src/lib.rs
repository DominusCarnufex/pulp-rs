#![crate_type = "dylib"]
#![crate_name = "pulp"]

//#![feature(alloc, heap_api, stmt_expr_attributes)]
#![feature(stmt_expr_attributes)]

mod utils;

mod md5;
use md5::*;

mod header;
use header::*;

//mod heapvar;
//use heapvar::*;

#[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
pub mod v0_1_0;

#[cfg(test)]
mod tests;

pub fn version(bytecode : &[u8]) -> Result<(u8, u8, u8), String>    {
    if bytecode.len() < HEADER_LENGTH   {
        return Err("Le fichier n’est pas un bytecode PULP : \
                    taille insuffisante.".to_string());
    }

    let header = match header(&bytecode)    {
        Ok(a)  => a,
        Err(e) => return Err(e)
    };

    if header.md5 != md5(&bytecode[HEADER_LENGTH..])    {
        let (a0, b0, c0, d0) = header.md5;
        let (a1, b1, c1, d1) = md5(&bytecode[HEADER_LENGTH..]);
        return Err(format!("Fichier corrompu : condensat MD5 incorrect.\n\
                    Trouvé :  0x{:x} 0x{:x} 0x{:x} 0x{:x}.\n\
                    Attendu : 0x{:x} 0x{:x} 0x{:x} 0x{:x}.",
                    a0, b0, c0, d0, a1, b1, c1, d1)
               );
    }

    if header.vers_maj == 0 && header.vers_min == 1 &&
        header.vers_patch == 0
    {
        // Si l’interpréteur a été compilé sans supporter cette version du
        // *bytecode*, inutile d’aller plus loin.
        #[cfg(not(any(feature = "v0_x", feature = "v0_1_x",
            feature = "v0_1_0")))]
        return Err(format!("Version du bytecode non supportée ({}.{}.{}). \
                        Compiler avec des options différentes pour l’obtenir.",
                        header.vers_maj, header.vers_min, header.vers_patch)
               );

        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        return Ok((0, 1, 0));
    } // End of v0.1.0.

    Err(format!("Version inconnue du bytecode ({}.{}.{}).",
            header.vers_maj, header.vers_min, header.vers_patch)
    )
} // End of version() function.
