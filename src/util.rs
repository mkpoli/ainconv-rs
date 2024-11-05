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

// Define the SplitIntoWords trait with the split_into_words method
pub trait SplitIntoWords {
    fn split_into_words(&self) -> Vec<String>;
}

// Implement the SplitIntoWords trait for the String type
impl SplitIntoWords for String {
    fn split_into_words(&self) -> Vec<String> {
        // Return an empty vector if the string is empty
        if self.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new(); // To store the resulting words
        let mut current = String::new(); // To build the current word
        let mut chars = self.chars(); // Create an iterator over the characters

        // Initialize with the first character
        let first_char = chars.next().unwrap();
        let mut current_is_letter = first_char.is_ainu_letter();
        current.push(first_char);

        // Iterate over the remaining characters
        for c in chars {
            let is_letter = c.is_ainu_letter();
            if is_letter == current_is_letter {
                // If the current character has the same classification, append it
                current.push(c);
            } else {
                // Classification changed; push the current word to result
                result.push(current.clone());
                current = c.to_string(); // Start a new word with the current character
                current_is_letter = is_letter; // Update the classification flag
            }
        }

        // Push the last word to the result
        result.push(current);

        result
    }
}

impl SplitIntoWords for &str {
    fn split_into_words(&self) -> Vec<String> {
        self.to_string().split_into_words()
    }
}

#[test]
fn test_split_into_words() {
    let text = String::from("Hello、世界! This is Rust.");
    assert_eq!(
        text.split_into_words(),
        vec!["Hello", "、", "世界", "! ", "This", " ", "is", " ", "Rust", "."]
    );
}

pub trait IsLetter {
    /// Check if the character is a letter in the Ainu script, including all possible scripts
    fn is_ainu_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_ainu_letter(&self) -> bool {
        self.is_alphabetic() || "\u{3099}\u{309A}\u{309B}\u{309C}\u{FF9E}\u{FF9F}".contains(*self)
    }
}

#[test]
fn test_is_ainu_letter() {
    assert_eq!('a'.is_ainu_letter(), true);
    assert_eq!('ア'.is_ainu_letter(), true);
    assert_eq!('あ'.is_ainu_letter(), true);
    assert_eq!('🐱'.is_ainu_letter(), false);
}
