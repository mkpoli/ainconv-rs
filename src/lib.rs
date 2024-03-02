/// Writing systems supported by the library.
#[derive(Debug, PartialEq)]
pub enum Script {
    /// Katakana - カタカナ
    Kana,
    /// Latin - Used for most Western languages
    Latn,
    /// Cyrillic - Used by many Slavic languages such as Russian
    Cyrl,
    // TODO: /// Hangul - The Korean alphabet
    // Hang,
    /// Mixed - A mix of different scripts
    Mixed,
    /// Unknown - Script could not be determined
    Unknown,
}

mod util;

mod conversion {
    pub mod cyrillic;
    // pub mod hangul;
    pub mod katakana;
    pub mod latin;
}

mod syllable;
pub use syllable::separate;

mod detection;
pub use detection::detect;

pub use conversion::cyrillic::{convert_cyrl_to_latn, convert_latn_to_cyrl};
pub use conversion::katakana::{convert_kana_to_latn, convert_latn_to_kana};

pub fn convert_cyrl_to_kana(cyrl: &str) -> String {
    convert_latn_to_kana(&convert_cyrl_to_latn(cyrl))
}

pub fn convert_kana_to_cyrl(kana: &str) -> String {
    convert_latn_to_cyrl(&convert_kana_to_latn(kana))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        assert_eq!(detect("aynu"), Script::Latn);
        assert_eq!(detect("アイヌ"), Script::Kana);
        assert_eq!(detect("айну"), Script::Cyrl);
        assert_eq!(detect("Aynuイタㇰ"), Script::Mixed);
        assert_eq!(detect("愛努"), Script::Unknown);
    }
}
