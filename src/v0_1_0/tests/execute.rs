use super::super::segments::{Const, Opcode, Segment};
use super::super::execute::Environment;

#[test]
fn pop()    {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Pop]
    };

    let sta = vec![Const::Int(42), Const::Int(79)];
    let env = vec![Environment::new()];

    let s = match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };

    let expected = vec![Const::Int(42)];

    assert_eq!(expected, s);
}

#[test]
#[should_panic(expected = "dépiler sur une pile vide")]
fn bad_pop()    {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Pop]
    };

    let sta = Vec::new();
    let env = vec![Environment::new()];

    match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };
}

#[test]
fn rot2()   {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Rot2]
    };

    let sta = vec![Const::Int(42), Const::Int(79)];
    let env = vec![Environment::new()];

    let s = match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };

    let expected = vec![Const::Int(79), Const::Int(42)];

    assert_eq!(expected, s);
}

#[test]
#[should_panic(expected = "Rot2 sur une pile contenant moins")]
fn bad_rot2()   {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Rot2]
    };

    let sta = vec![Const::Int(42)];
    let env = vec![Environment::new()];

    match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };
}

#[test]
fn rot3()   {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Rot3]
    };

    let sta = vec![Const::Int(42), Const::Int(79), Const::Int(82)];
    let env = vec![Environment::new()];

    let s = match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };

    let expected = vec![Const::Int(79), Const::Int(82), Const::Int(42)];

    assert_eq!(expected, s);
}

#[test]
#[should_panic(expected = "Rot3 sur une pile contenant moins")]
fn bad_rot3()   {
    let seg = Segment::Code {
        symbol_table : Vec::new(),
        const_table  : Vec::new(),
        code         : vec![Opcode::Rot3]
    };

    let sta = vec![Const::Int(42), Const::Int(79)];
    let env = vec![Environment::new()];

    match seg.execute((sta, env))   {
        Ok((a, _))  => a,
        Err(e)      => panic!("{}", e)
    };
}

