use super::super::segments::*;

#[test]
fn segment()    {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00, // (Size = 20 octets)
        0x01, 0x00, 0x00, 0x00, // (Type = code)
        0x04, 0x00, 0x00, 0x00, // (Empty code segment)
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
    ];

    let s = match segments(&vec)    {
        Ok(a)  => a,
        Err(e) => panic!("{}", e)
    };

    let expected = vec![
        Segment::Code   {
            symbol_table : Vec::new(),
            const_table  : Vec::new(),
            code         : Vec::new()
        }
    ];

    assert_eq!(expected, s);
}

#[test]
#[should_panic(expected = "taille incorrecte")]
fn bad_segment_size()   {
    let vec = vec![
        0x00, 0x00, 0x00, 0x80, // (Size = 2⁶³)
        0x01, 0x00, 0x00, 0x00, // (Type = code)
        0x04, 0x00, 0x00, 0x00, // (Empty code segment)
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

#[test]
#[should_panic(expected = "type inconnu")]
fn bad_segment_type()   {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00, // (Size = 20 octets)
        0x00, 0x00, 0x00, 0x00, // (Type = none)
        0x04, 0x00, 0x00, 0x00, // (Empty code segment)
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}
