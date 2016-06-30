use super::super::md5;

#[test]
fn md5()    {
    let vec = vec![
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

    let m = md5::md5(&vec);

    assert_eq!((0xcb2de7c3, 0x2015c523, 0x646219ae, 0x9d8a9a5b), m);
}
