use ::heapvar::HeapVar;
use ::header::HEADER_LENGTH;

mod segments;
use self::segments::*;

mod execute;
use self::execute::*;

#[cfg(test)]
mod tests;

pub enum PulpResult<A, B, C>  {
    Ok(A),
    ProgErr(B),
    CompErr(C)
}

pub fn run(bytecode : &[u8])
    -> PulpResult<Option<Const>, (usize, ProgState), String>
{
    let segments = match segments(&bytecode[HEADER_LENGTH..])   {
        Ok(a)  => a,
        Err(e) => return PulpResult::CompErr(e)
    };

    for s in segments   {
        if s.name() == "main"   {
            match s.execute((Vec::new(), vec![Environment::new()])) {
                Ok((mut stack, env)) => {
                    if stack.len() == 0 {
                        return PulpResult::Ok(None);
                    }

                    let ts = stack.pop().unwrap();

                    if let Const::Abort(err) = ts   {
                        return PulpResult::ProgErr((err, (stack, env)));
                    }

                    return PulpResult::Ok(Some(ts));
                },
                Err(e) => return PulpResult::CompErr(e)
            }
        } // End of if.
    } // End of loop.

    return PulpResult::CompErr("Exécution impossible : aucun segment \
        `main` disponible.".to_string());
} // End of run() function.

#[repr(C)]
pub struct COption  {
    valid : bool,
    value : i64
}

#[repr(C)]
pub struct CPulpResult  {
    which : u8,
    value : HeapVar,
}

#[no_mangle]
pub extern fn run_v0_1_0(entree : HeapVar) -> CPulpResult   {
    let vec = match entree.vector::<u8>()   {
        Ok(a)  => a,
        Err(e) => {
            let string  = format!("Entrée incorrecte : {}", e);
            let heapvar = match HeapVar::from_string(string)    {
                Ok(a)  => a,
                Err(e) => panic!("Panique totale : {}", e)
            };
            return CPulpResult { which : 2, value : heapvar};
        }
    };

    match run(&vec) {
        PulpResult::Ok(a)                  => {
            if let Some(c) = a  {
                if let Const::Int(i) = c    {
                    let res = COption { valid : true, value : i };
                    let heapvar = match HeapVar::from(res)  {
                        Ok(a)  => a,
                        Err(e) => panic!("Panique totale : {}", e)
                    };
                    return CPulpResult { which : 0, value : heapvar};
                } else {
                    panic!("Panique totale : censément impossible.");
                }
            } else {
                let res = COption { valid : false, value : 0 };
                let heapvar = match HeapVar::from(res)  {
                    Ok(a)  => a,
                    Err(e) => panic!("Panique totale : {}", e)
                };
                return CPulpResult { which : 0, value : heapvar};
            }
        }, // End of Ok branch.
        PulpResult::ProgErr((err, (_, _))) => {
            let heapvar = match HeapVar::from(err)  {
                Ok(a)  => a,
                Err(e) => panic!("Panique totale : {}", e)
            };
            return CPulpResult { which : 1, value : heapvar};
        },
        PulpResult::CompErr(e)             => {
            let heapvar = match HeapVar::from_string(e) {
                Ok(a)  => a,
                Err(e) => panic!("Panique totale : {}", e)
            };
            return CPulpResult { which : 2, value : heapvar};
        },
    } // End of match.
} // End of run_v0_1_0() function.
