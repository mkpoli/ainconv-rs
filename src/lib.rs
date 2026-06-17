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

/// Options that tweak how punctuation is normalized during conversion.
///
/// Construct from [`Default`] and override only the toggles you need:
///
/// ```
/// use ainconv::ConversionOptions;
/// let opts = ConversionOptions { ellipsis_to_ascii: true, ..Default::default() };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConversionOptions {
    /// When `true`, the horizontal ellipsis `…` (U+2026) is rewritten as three
    /// ASCII full stops `...` during Katakana → Latin conversion.
    ///
    /// Defaults to `false`, leaving `…` intact: it is a single, semantically
    /// correct character, so the rewrite is opt-in.
    pub ellipsis_to_ascii: bool,

    /// When `true`, Latin → Katakana keeps `ヰ` (wi) instead of spelling it out
    /// as `ウィ`, e.g. `wina` → `ヰナ` instead of `ウィナ`. Defaults to `false`.
    pub use_wi: bool,

    /// When `true`, Latin → Katakana keeps `ヱ` (we) instead of spelling it out
    /// as `ウェ`, e.g. `weni` → `ヱニ` instead of `ウェニ`. Defaults to `false`.
    pub use_we: bool,

    /// When `true`, Latin → Katakana keeps `ヲ` (wo) instead of spelling it out
    /// as `ウォ`. Defaults to `false`.
    pub use_wo: bool,

    /// When `true`, Latin → Katakana keeps the small `ィ` for the `-y` coda
    /// instead of `イ`, e.g. `kay` → `カィ` instead of `カイ`. Defaults to `false`.
    pub use_small_i: bool,

    /// When `true`, Latin → Katakana keeps the small `ゥ` for the `-w` coda
    /// instead of `ウ`, e.g. `kew` → `ケゥ` instead of `ケウ`. Defaults to `false`.
    pub use_small_u: bool,

    /// When `true`, Latin → Katakana keeps `ㇴ` for the `-n` coda instead of `ン`,
    /// e.g. `mun` → `ムㇴ` instead of `ムン`. Defaults to `false`.
    pub use_small_n: bool,
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
pub use conversion::katakana::{
    convert_kana_to_latn, convert_kana_to_latn_with_options, convert_latn_to_kana,
    convert_latn_to_kana_with_options,
};

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
