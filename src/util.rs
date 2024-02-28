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