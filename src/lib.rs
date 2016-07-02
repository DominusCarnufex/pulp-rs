#![allow(dead_code)]
#![feature(stmt_expr_attributes)]

mod utils;

mod md5;
use md5::*;

mod header;
use header::*;

#[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
mod v0_1_0;

#[cfg(test)]
mod tests;

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
        let (a0, b0, c0, d0) = header.md5;
        let (a1, b1, c1, d1) = md5(&bytecode[HEADER_LENGTH..]);
        return Err(format!("Fichier corrompu : condensat MD5 incorrect.\n\
                    Trouvé :  0x{:x} 0x{:x} 0x{:x} 0x{:x}.\n\
                    Attendu : 0x{:x} 0x{:x} 0x{:x} 0x{:x}.",
                    a0, b0, c0, d0, a1, b1, c1, d1)
               );
    }

    /* EXÉCUTION DE LA VERSION 0.1.0 DU *BYTECODE*. */
    /* Les blocs #[cfg()] ne sont pas acceptés avant un `if`, ce qui oblige à
       en placer un sur chaque élément de niveau inférieur. */

    if header.vers_maj == 0 && header.vers_min == 1 &&
        header.vers_patch == 0
    {
        // Si l’interpréteur a été compilé sans supporter cette version du
        // *bytecode*, inutile d’aller plus loin.
        if !(cfg!(feature = "v0_x") || cfg!(feature = "v0_1_x") ||
            cfg!(feature = "v0_1_0"))
        {
            return Err(format!("Version du bytecode non supportée ({}.{}.{}). \
                        Compiler avec des options différentes pour l’obtenir.",
                        header.vers_maj, header.vers_min, header.vers_patch)
                   );
        }

        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        let segments = match v0_1_0::segments(&bytecode[HEADER_LENGTH..])   {
            Ok(a)  => a,
            Err(e) => return Err(e)
        };

        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        for s in segments   {
            if s.name() == "main"   {
                match s.execute((Vec::new(), vec![v0_1_0::Environment::new()])) {
                    Ok((mut stack, _)) => {
                        if stack.len() > 0  {
                            let v0_1_0::Const::Int(ts) = stack.pop().unwrap();
                            println!("Sommet de la pile : {}.", ts);
                        } else {
                            println!("Pile vide.");
                        }

                        return Ok(());
                    },
                    Err(e)             => return Err(e)
                } // End of match.
            } // End of if.
        } // End of loop.

        #[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
        return Err("Exécution impossible : aucun segment \
            `main` disponible.".to_string());
    } // End of v0.1.0.

    Err(format!("Version inconnue du bytecode ({}.{}.{}).",
        header.vers_maj, header.vers_min, header.vers_patch)
    )
} // End of run() function.
