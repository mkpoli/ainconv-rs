use unicode_normalization::UnicodeNormalization;

pub fn is_katakana(c: char) -> bool {
    ('\u{30A1}'..='\u{31FF}').contains(&c)
}

pub fn is_cyrillic(c: char) -> bool {
    ('\u{0400}'..='\u{04FF}').contains(&c)
}

// pub fn is_hangul(c: char) -> bool {
//     let re = Regex::new(r"\p{Script_Extensions=Hangul}").unwrap();
//     re.is_match(c.to_string().as_str())
// }

pub fn remove_acute_accent(text: &str) -> String {
    text.nfd()
        .filter(|c| *c as u32 != 0x301)
        .nfc()
        .collect::<String>()
}

#[test]
fn test_remove_accent() {
    // Remove from composed
    assert_eq!(remove_acute_accent("á"), "a");

    // Remove from decomposed
    assert_eq!(remove_acute_accent("á"), "a");

    // Remove from decomposed with other characters
    assert_eq!(remove_acute_accent("cápe"), "cape");

    // Remove from a multiple accented characters
    assert_eq!(remove_acute_accent("pā́"), "pā");

    // Remove from a composed character with a non-accented character
    assert_eq!(remove_acute_accent("cá̄"), "cā");
}
