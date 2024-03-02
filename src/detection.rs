use crate::util::{is_cyrillic, is_katakana};
use crate::Script;
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

    if [has_latin, has_cyrillic, has_kana]
        .iter()
        .filter(|&&b| b)
        .count()
        > 1
    {
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
