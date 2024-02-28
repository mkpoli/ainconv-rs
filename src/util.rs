use regex::Regex;

pub fn is_katakana(c: char) -> bool {
    let re = Regex::new(r"\p{Script_Extensions=Katakana}").unwrap();
    re.is_match(c.to_string().as_str())
}

pub fn is_cyrillic(c: char) -> bool {
    let re = Regex::new(r"\p{Script_Extensions=Cyrillic}").unwrap();
    re.is_match(c.to_string().as_str())
}

// pub fn is_hangul(c: char) -> bool {
//     let re = Regex::new(r"\p{Script_Extensions=Hangul}").unwrap();
//     re.is_match(c.to_string().as_str())
// }