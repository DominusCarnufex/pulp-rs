#![allow(dead_code)]
#![feature(stmt_expr_attributes)]

mod utils;

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

    // TODO Vérification MD5.

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
