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

const TEST_CASES: [(&str, &'static [&str], &str, &str, &str, &str); 16] = [
    ("", &[], "", "", "", ""),
    ("aynu", &["ay", "nu"], "アイヌ", "айну", "애누", "ainu"),
    ("itak", &["i", "tak"], "イタㇰ", "итак", "이닥", "itak"),
    (
        "maciya",
        &["ma", "ci", "ya"],
        "マチヤ",
        "мация", /* TODO: мачия? */
        "마지야",
        "maciya",
    ),
    (
        "acapo",
        &["a", "ca", "po"],
        "アチャポ",
        "ацапо", /* TODO: ачапо? */
        "아자포",
        "acapo",
    ),
    (
        "aynuitak",
        &["ay", "nu", "i", "tak"],
        "アイヌイタㇰ",
        "айнуитак",
        "애누이닥",
        "ainuitak",
    ),
    ("sinep", &["si", "nep"], "シネㇷ゚", "синэп", "시넙", "sinep"),
    (
        "ruunpe",
        &["ru", "un", "pe"],
        "ルウンペ",
        "руунпэ",
        "루운버",
        "ruunpe",
    ),
    (
        "wenkur",
        &["we", "n", "kur"],
        "ウェンクㇽ",
        "вэнкур",
        "펀굴",
        "wenkur",
    ),
    (
        "pekanke",
        &["pe", "kan", "ke"],
        "ペカンケ",
        "пэканкэ",
        "버간거",
        "pekanke",
    ),
    (
        "eramuskare",
        &["e", "ra", "mus", "ka", "re"],
        "エラムㇱカレ",
        "эрамускарэ",
        "어라뭇가러",
        "eramuskare",
    ),
    (
        "hioy’oy",
        &["hi", "oy", "oy"],
        "ヒオイオイ",
        "хиойой",
        "히외외",
        "hioioi",
    ),
    (
        "irankarapte",
        &["i", "ran", "ka", "rap", "te"],
        "イランカラㇷ゚テ",
        "иранкараптэ",
        "이란가랍더",
        "irankarapte",
    ),
    (
        "iyairaykere",
        &["i", "ya", "yi", "ray", "ke", "re"],
        "イヤイライケレ",
        "ияирайкэрэ",
        "이야이래거러",
        "iyairaikere",
    ),
    (
        "yayrayke",
        &["yay", "ray", "ke"],
        "ヤイライケ",
        "яйрайкэ",
        "얘래거",
        "yairaike",
    ),
    (
        "keyaykosiramsuypa",
        &["ke", "yay", "ko", "si", "ram", "suy", "pa"],
        "ケヤイコシラㇺスイパ",
        "кэяйкосирамсуйпа",
        "거얘고시람쉬바",
        "keyaikosiramsuipa",
    ),
];

#[test]
fn test_convert_latn_to_kana() {
    for case in CASES.iter() {
        assert_eq!(convert_latn_to_kana(&case.latn), case.kana.to_owned());
    }

    for (latn, _, kana, _, _, _) in &TEST_CASES {
        assert_eq!(convert_latn_to_kana(latn), kana.to_owned());
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
        assert_eq!(convert_kana_to_latn(&case.kana), case.latn.to_owned());
    }

    for (_, _, kana, _, _, latn) in &TEST_CASES {
        assert_eq!(convert_kana_to_latn(kana), latn.to_owned());
    }
    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Kana => assert_eq!(convert_kana_to_latn(&case.kana), case.kana.to_owned()),
            _ => (),
        }
    }
}

#[test]
fn test_convert_latn_to_cyrl() {
    for case in CASES.iter() {
        assert_eq!(convert_latn_to_cyrl(&case.latn), case.cyrl.to_owned());
    }

    for (latn, _, _, cyrl, _, _) in &TEST_CASES {
        assert_eq!(convert_latn_to_cyrl(latn), cyrl.to_owned());
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

    for (_, _, _, cyrl, _, latn) in &TEST_CASES {
        assert_eq!(convert_cyrl_to_latn(cyrl), latn.to_owned());
    }

    for case in ROBUSTNESS.iter() {
        match case.from {
            From::Cyrl => assert_eq!(convert_cyrl_to_latn(&case.cyrl), case.latn.to_owned()),
            _ => (),
        }
    }
}
