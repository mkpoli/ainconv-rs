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
pub use conversion::katakana::{
    convert_kana_to_latn, convert_latn_to_kana, convert_latn_to_kana_with_options, KanaOptions,
};

pub fn convert_cyrl_to_kana(cyrl: &str) -> String {
    convert_latn_to_kana(&convert_cyrl_to_latn(cyrl))
}

pub fn convert_cyrl_to_kana_with_options(cyrl: &str, options: &KanaOptions) -> String {
    convert_latn_to_kana_with_options(&convert_cyrl_to_latn(cyrl), options)
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

    /// Drive the shared option catalog from the vendored `ainconv-tests` vectors:
    /// every `variants` entry in test_cases.json must reproduce exactly.
    #[test]
    fn test_latn_to_kana_options() {
        let data = include_str!("../tests/cases/test_cases.json");
        let cases: serde_json::Value = serde_json::from_str(data).unwrap();
        let flag =
            |o: &serde_json::Value, k: &str| o.get(k).and_then(|b| b.as_bool()).unwrap_or(false);

        for case in cases.as_array().unwrap() {
            let latn = case["latn"].as_str().unwrap();
            let Some(variants) = case.get("variants").and_then(|v| v.as_array()) else {
                continue;
            };
            for variant in variants {
                let o = &variant["options"];
                let options = KanaOptions {
                    use_wi: flag(o, "useWi"),
                    use_we: flag(o, "useWe"),
                    use_wo: flag(o, "useWo"),
                    use_small_i: flag(o, "useSmallI"),
                    use_small_u: flag(o, "useSmallU"),
                    use_small_n: flag(o, "useSmallN"),
                };
                let expected = variant["kana"].as_str().unwrap();
                let got = convert_latn_to_kana_with_options(latn, &options);
                assert_eq!(got, expected, "latn={latn}, options={o}");
            }
        }
    }
}
