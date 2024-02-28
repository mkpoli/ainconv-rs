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
use util::{is_cyrillic, is_katakana};

mod conversion {
    pub mod cyrillic;
    // pub mod hangul;
    pub mod katakana;
    pub mod latin;
}

mod syllable;
pub use syllable::separate;

pub use conversion::cyrillic::{convert_latn_to_cyrl, convert_cyrl_to_latn};
pub use conversion::katakana::{convert_latn_to_kana, convert_kana_to_latn};

pub fn convert_cyrl_to_kana(cyrl: &str) -> String {
    convert_latn_to_kana(&convert_cyrl_to_latn(cyrl))
}

pub fn convert_kana_to_cyrl(kana: &str) -> String {
    convert_latn_to_cyrl(&convert_kana_to_latn(kana))
}

/// Detects the script type of a given Ainu language string.
///
/// This function categorizes the script into one of several types based on the characters present in the string.
/// It supports Latin, Cyrillic, Katakana, and Hangul scripts, and can also identify mixed or unknown scripts.
///
/// # Arguments
///
/// * `s` - The text string to be analyzed for script type.
///
/// # Returns
///
/// * `Script` - The detected script type:
///   * `Kana` for Katakana
///   * `Cyrl` for Cyrillic
///   * `Latn` for Latin
///   * `Mixed` if multiple scripts are detected (excluding Hangul)
///   * `Unknown` if no script is detected
///
/// # Example
///
/// ```
/// use ainconv::{detect, Script};
/// let script = detect("アイヌ");
/// assert_eq!(script, Script::Kana);
/// ```
pub fn detect(s: &str) -> Script {
    let has_latin = s.chars().any(|c| c.is_alphabetic() && c.is_ascii());
    let has_cyrillic = s.chars().any(|c| c.is_alphabetic() && is_cyrillic(c));
    let has_kana = s.chars().any(|c| c.is_alphabetic() && is_katakana(c));
    // let has_hangul = s.chars().any(|c| c.is_alphabetic() && is_hangul(c));

    if [has_latin, has_cyrillic, has_kana].iter().filter(|&&b| b).count() > 1 {
        Script::Mixed
    } else if has_kana {
        Script::Kana
    } else if has_cyrillic {
        Script::Cyrl
    // } else if has_hangul {
    //     Script::Hang
    } else if has_latin {
        Script::Latn
    } else {
        Script::Unknown
    }
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
