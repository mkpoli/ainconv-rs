use ainconv::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const TEST_CASES_JSON: &str = include_str!("./cases/test_cases.json");
const ROBUSTNESS_JSON: &str = include_str!("./cases/robustness.json");
const OPTIONS_CATALOG_JSON: &str = include_str!("./cases/options.schema.json");

lazy_static! {
    static ref CASES: Vec<Case> = serde_json::from_str(TEST_CASES_JSON).unwrap();
    static ref ROBUSTNESS: Vec<RobustnessCase> = serde_json::from_str(ROBUSTNESS_JSON).unwrap();
    static ref OPTIONS_CATALOG: OptionCatalog =
        serde_json::from_str(OPTIONS_CATALOG_JSON).unwrap();
}

/// The shared canonical option catalog (`options.schema.json`).
#[derive(Debug, Deserialize)]
struct OptionCatalog {
    options: Vec<OptionSpecMeta>,
}

#[derive(Debug, Deserialize)]
struct OptionSpecMeta {
    key: String,
}

/// Canonical camelCase keys of the options ainconv-rs implements. Kept in sync
/// with [`ConversionOptions`]; the parity test below asserts this equals the
/// shared catalog so an option added in any language fails this build until
/// Rust implements it too.
const SUPPORTED_OPTIONS: &[&str] = &[
    "ellipsisToAscii",
    "useWi",
    "useWe",
    "useWo",
    "useSmallI",
    "useSmallU",
    "useSmallN",
];

#[test]
fn test_option_catalog_parity() {
    use std::collections::BTreeSet;
    let catalog: BTreeSet<&str> = OPTIONS_CATALOG.options.iter().map(|o| o.key.as_str()).collect();
    let supported: BTreeSet<&str> = SUPPORTED_OPTIONS.iter().copied().collect();
    assert_eq!(
        supported, catalog,
        "ainconv-rs option set drifted from the shared options.schema.json catalog"
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct Case {
    latn: String,
    kana: String,
    hang: String,
    cyrl: String,
    syllables: Vec<String>,
    #[serde(rename = "latnLossy")]
    latn_lossy: String,
    #[serde(default)]
    variants: Vec<Variant>,
}

/// A conversion-options scenario sharing a base case's input. Output fields are
/// optional: only the ones present are asserted, so a variant can target a
/// single direction. Option keys are camelCase to match the shared JSON.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OptionsSpec {
    #[serde(default)]
    ellipsis_to_ascii: bool,
    #[serde(default)]
    use_wi: bool,
    #[serde(default)]
    use_we: bool,
    #[serde(default)]
    use_wo: bool,
    #[serde(default)]
    use_small_i: bool,
    #[serde(default)]
    use_small_u: bool,
    #[serde(default)]
    use_small_n: bool,
}

impl OptionsSpec {
    fn to_options(&self) -> ConversionOptions {
        ConversionOptions {
            ellipsis_to_ascii: self.ellipsis_to_ascii,
            use_wi: self.use_wi,
            use_we: self.use_we,
            use_wo: self.use_wo,
            use_small_i: self.use_small_i,
            use_small_u: self.use_small_u,
            use_small_n: self.use_small_n,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Variant {
    options: OptionsSpec,
    kana: Option<String>,
    #[serde(rename = "latnLossy")]
    latn_lossy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum From {
    Latn,
    Kana,
    Cyrl,
    Hang,
}

#[derive(Debug, Serialize, Deserialize)]
struct RobustnessCase {
    from: From,
    #[serde(rename = "Kana")]
    kana: String,
    #[serde(rename = "Cyrl")]
    cyrl: String,
    #[serde(rename = "Hang")]
    hang: String,
    #[serde(rename = "Latn")]
    latn: String,
}

/* maciya -> мация TODO: мачия? */
/* acapo -> ацапо TODO: ачапо? */

#[test]
fn test_convert_latn_to_kana() {
    for case in CASES.iter() {
        assert_eq!(convert_latn_to_kana(&case.latn), case.kana.to_owned());
    }

    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Latn => assert_eq!(convert_latn_to_kana(&case.latn), case.kana.to_owned()),
            _ => (),
        }
    }
}

/// Exercises the shared `variants` blocks: each variant reuses its base case's
/// input but applies conversion options and overrides the expected output.
#[test]
fn test_conversion_options_variants() {
    let mut checked = 0;
    for case in CASES.iter() {
        for variant in case.variants.iter() {
            let options = variant.options.to_options();

            if let Some(expected_kana) = &variant.kana {
                assert_eq!(
                    convert_latn_to_kana_with_options(&case.latn, &options),
                    *expected_kana,
                    "latn->kana variant {:?} for {:?}",
                    variant.options,
                    case.latn
                );
                checked += 1;
            }

            if let Some(expected_latn) = &variant.latn_lossy {
                assert_eq!(
                    convert_kana_to_latn_with_options(&case.kana, &options),
                    *expected_latn,
                    "kana->latn variant {:?} for {:?}",
                    variant.options,
                    case.kana
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no option variants were exercised");
}

#[test]
fn test_convert_kana_to_latn() {
    for case in CASES.iter() {
        assert_eq!(convert_kana_to_latn(&case.kana), case.latn_lossy.to_owned());
    }

    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Kana => assert_eq!(convert_kana_to_latn(&case.kana), case.latn.to_owned()),
            _ => (),
        }
    }
}

#[test]
fn test_convert_latn_to_cyrl() {
    for case in CASES.iter() {
        assert_eq!(convert_latn_to_cyrl(&case.latn), case.cyrl.to_owned());
    }

    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Latn => assert_eq!(convert_latn_to_cyrl(&case.latn), case.cyrl.to_owned()),
            _ => (),
        }
    }
}

#[test]
fn test_convert_cyrl_to_latn() {
    for case in CASES.iter() {
        assert_eq!(convert_cyrl_to_latn(&case.cyrl), case.latn.to_owned());
    }

    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Cyrl => assert_eq!(convert_cyrl_to_latn(&case.cyrl), case.latn.to_owned()),
            _ => (),
        }
    }
}
