use ainconv::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const TEST_CASES_JSON: &str = include_str!("./cases/test_cases.json");
const ROBUSTNESS_JSON: &str = include_str!("./cases/robustness.json");

lazy_static! {
    static ref CASES: Vec<Case> = serde_json::from_str(TEST_CASES_JSON).unwrap();
    static ref ROBUSTNESS: Vec<RobustnessCase> = serde_json::from_str(ROBUSTNESS_JSON).unwrap();
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
