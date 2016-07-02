use ::version;

// Un programme de seulement 4 octets ne contient pas
// un en-tête complet, et ne peut donc être valide.
#[test]
#[should_panic(expected = "taille insuffisante")]
fn too_small()  {
    let vec = vec![0x50, 0x75, 0x4c, 0x50];

    match version(&vec) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// Un programme valide, à l’exception du fait que le
// condensat MD5 est faux, signalant ainsi une perte
// d’intégrité du fichier.
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
        0x32, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x6d, 0x61, 0x69, 0x6e,
        0x04, 0x00, 0x00, 0x00,
        0x16, 0x00, 0x02, 0x00,
          0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
        0x0b, 0x00, 0x03, 0x00,
          0x0a, 0x00, 0x00,
          0x0a, 0x01, 0x00,
          0x30
    ];

    match version(&vec) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// Un programme complet utilisant la version 0.1.0 du
// *bytecode*. Si l’interpréteur a été compilé en autorisant
// cette version du *bytecode*, le test doit réussir.
#[test]
#[cfg(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0"))]
fn good_version_v0_1_0()    {
    let vec = vec![
        0x50, 0x55, 0x4c, 0x50,
        0x00, 0x01, 0x00,
        0x00,
        0x57, 0x6e, 0xbc, 0xfa,
        0x32, 0x00, 0x7b, 0xf9, 0x73, 0xca, 0x8b, 0x5f,
        0x09, 0xe0, 0x54, 0x09, 0x3a, 0xab, 0xf2, 0x60,
        0x32, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x6d, 0x61, 0x69, 0x6e,
        0x04, 0x00, 0x00, 0x00,
        0x16, 0x00, 0x02, 0x00,
          0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,    
        0x0b, 0x00, 0x03, 0x00,
          0x0a, 0x00, 0x00,
          0x0a, 0x01, 0x00,
          0x30
    ];

    match version(&vec) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// Pendant du précédent, pour quand la version 0.1.0 du
// *bytecode* n’a pas été activée à la compilation.
#[test]
#[cfg(not(any(feature = "v0_x", feature = "v0_1_x", feature = "v0_1_0")))]
#[should_panic(expected = "Version du bytecode non supportée")]
fn bad_version_v0_1_0() {
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

    match version(&vec) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// La version 0.0.1 du *bytecode* n’existe pas.
#[test]
#[should_panic(expected = "Version inconnue du bytecode")]
fn bad_version()    {
    let vec = vec![
        0x50, 0x55, 0x4c, 0x50,
        0x00, 0x00, 0x01,
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

    match version(&vec) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}
