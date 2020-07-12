use unicode_charname::CharName;

fn some_s(s: &str) -> Option<String> {
    Some(s.to_string())
}
#[test]
fn enumeration_names() {
    assert_eq!(
        some_s("LATIN CAPITAL LETTER A"),
        'A'.char_name().map(|x| x.to_string())
    );
    assert_eq!(
        some_s(
            "ARABIC LIGATURE UIGHUR KIRGHIZ YEH WITH HAMZA ABOVE WITH ALEF MAKSURA ISOLATED FORM"
        ),
        0xFBF9u32.char_name().map(|x| x.to_string())
    );
    assert_eq!(some_s("OX"), 0x1F402u32.char_name().map(|x| x.to_string()));
    assert_eq!(
        some_s("HANGUL JUNGSEONG O-E"),
        0x1180u32.char_name().map(|x| x.to_string())
    );
    assert_eq!(
        some_s("PRESENTATION FORM FOR VERTICAL RIGHT WHITE LENTICULAR BRAKCET"),
        0xFE18u32.char_name().map(|x| x.to_string())
    );
    /* FIXME
    assert_eq!(
        None,
        0x0009u32.property_name().map(|x| x.to_string())
    );
    assert_eq!(
        some_s("control-0009"),
        0x0009u32.char_name().map(|x| x.to_string())
    );
    */
}
