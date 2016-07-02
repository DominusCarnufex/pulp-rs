//use ::heapvar::*;
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
