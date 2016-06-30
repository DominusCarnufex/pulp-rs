use super::super::segments::*;

    /* TESTS DE LA FONCTION segments()
      *********************************/

// Plus petit segment reconnu comme valide.
#[test]
fn segment()    {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
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

// La taille prétendue (2⁶³ octets) est supérieure à la
// taille totale du bytecode.
#[test]
#[should_panic(expected = "taille incorrecte")]
fn bad_segment_size()   {
    let vec = vec![
        0x00, 0x00, 0x00, 0x80,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// Le type de segment 0x00 n’est pas défini.
#[test]
#[should_panic(expected = "type inconnu")]
fn bad_segment_type()   {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, // <-------
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

    /* TESTS DE LA FONCTION code_segment()
      *************************************/

// Le segment fourni est trop court pour pouvoir être un
// segment de code.
#[test]
#[should_panic(expected = "trop petit pour être valide")]
fn code_segment_too_short() {
    let vec = vec![
        0x10, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// La taille prétendue (21 octets) est supérieure à la
// taille réelle des différents éléments du segment.
#[test]
#[should_panic(expected = "taille fournie incohérente avec le segment")]
fn wrong_segment_size() {
    let vec = vec![
        0x15, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x00 // <-------------- Octet inutile.
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

    /***** TABLE DES SYMBOLES *****/

// Table de symboles contenant deux symboles : « réponse »
// et « Ångermanland », permettant de vérifier que les
// caractères Unicode sont bien reconnus.
#[test]
fn code_segment_symbols()   {
    let vec = vec![
        0x2b, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x1b, 0x00, 0x02, 0x00,
           0x08, 0x72, 0xc3, 0xa9, 0x70, 0x6f, 0x6e, 0x73, 0x65,
           0x0d, 0xC3, 0x85, 0x6e, 0x67, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x6c,
            0x61, 0x6e, 0x64,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    let seg = match segments(&vec)  {
        Ok(mut a)  => a.pop().unwrap(),
        Err(e)     => panic!("{}", e)
    };

    let expected = Segment::Code    {
            symbol_table : vec![
                            "réponse".to_string(),
                            "Ångermanland".to_string()
                           ],
            const_table  : Vec::new(),
            code         : Vec::new()
    };

    assert_eq!(expected, seg);
    
}

// La taille prétendue de la table des symboles (20
// octets) va au-delà de la fin du segment.
#[test]
#[should_panic(expected = "TS trop petite")]
fn code_segment_symbol_table_too_short_1()  {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x14, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// La taille prétendue de la table des symboles est de 3
// octets, alors qu’une table des symboles fait au moins
// 4 octets.
#[test]
#[should_panic(expected = "TS trop petite")]
fn code_segment_symbol_table_too_short_2()  {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// Le symbole présent à une taille de 0, ce qui n’est pas
// acceptable.
#[test]
#[should_panic(expected = "la taille doit être au moins de 1")]
fn code_segment_symbol_bad_size()   {
    let vec = vec![
        0x15, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x01, 0x00,
           0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// La suite d’octets 0xf0 0x82 0x82 0xac n’est pas de
// l’UTF-8 valide.
#[test]
#[should_panic(expected = "UTF-8 invalide")]
fn code_segment_symbol_bad_value()  {
    let vec = vec![
        0x19, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x09, 0x00, 0x01, 0x00,
           0x04, 0xf0, 0x82, 0x82, 0xac,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}

// La taille prétendue de la table des symboles est de 5
// octets, alors qu’elle en fait 4.
#[test]
#[should_panic(expected = "taille fournie incohérente avec la TS")]
fn code_segment_symbol_table_wrong_size()   {
    let vec = vec![
        0x14, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00
    ];

    match segments(&vec)    {
        Ok(_)  => {},
        Err(e) => panic!("{}", e)
    }
}
