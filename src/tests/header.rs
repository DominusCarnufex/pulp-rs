use super::super::header;

#[test]
fn header() {
    let vec = vec![
        0x50, 0x55, 0x4c, 0x50,
        0x00, 0x01, 0x00,
        0x00,
        0x57, 0x6e, 0xbc, 0xfa,
        0x82, 0x71, 0x3a, 0x3b, 0x49, 0x41, 0x98, 0x7c,
        0x15, 0x55, 0x41, 0x1d, 0x9e, 0xe4, 0xb6, 0x7b
    ];

    let h = match header::header(&vec)  {
        Ok(a)  => a,
        Err(e) => panic!("{}", e)
    };

    let md5 = (0x3b3a7182, 0x7c984149, 0x1d415515, 0x7bb6e49e);

    assert_eq!(0,          h.vers_maj);
    assert_eq!(1,          h.vers_min);
    assert_eq!(0,          h.vers_patch);
    assert_eq!(0xfabc6e57, h.timestamp);
    assert_eq!(md5,        h.md5);
}

#[test]
#[should_panic(expected = "nombre magique incorrect")]
fn bad_magic()  {
    let vec = vec![0x50, 0x75, 0x4c, 0x50];
    match header::header(&vec)  {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}
