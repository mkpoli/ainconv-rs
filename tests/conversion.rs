use ainconv::*;

const TEST_CASES: [(&str, &'static [&str], &str, &str, &str, &str); 14] = [
    ("", &[], "", "", "", ""),
    ("aynu", &["ay", "nu"], "アイヌ", "айну", "애누", "ainu"),
    ("itak", &["i", "tak"], "イタㇰ", "итак", "이닥", "itak"),
    ("aynuitak", &["ay", "nu", "i", "tak"], "アイヌイタㇰ", "айнуитак", "애누이닥", "ainuitak"),
    ("sinep", &["si", "nep"], "シネㇷ゚", "синэп", "시넙", "sinep"),
    ("ruunpe", &["ru", "un", "pe"], "ルウンペ", "руунпэ", "루운버", "ruunpe"),
    ("wenkur", &["we", "n", "kur"], "ウェンクㇽ", "вэнкур", "펀굴", "wenkur"),
    ("pekanke", &["pe", "kan", "ke"], "ペカンケ", "пэканкэ", "버간거", "pekanke"),
    ("eramuskare", &["e", "ra", "mus", "ka", "re"], "エラムㇱカレ", "эрамускарэ", "어라뭇가러", "eramuskare"),
    ("hioy’oy", &["hi", "oy", "oy"], "ヒオイオイ", "хиойой", "히외외", "hioioi"),
    ("irankarapte", &["i", "ran", "ka", "rap", "te"], "イランカラㇷ゚テ", "иранкараптэ", "이란가랍더", "irankarapte"),
    ("iyairaykere", &["i", "ya", "yi", "ray", "ke", "re"], "イヤイライケレ", "ияирайкэрэ", "이야이래거러", "iyairaikere"),
    ("yayrayke", &["yay", "ray", "ke"], "ヤイライケ", "яйрайкэ", "얘래거", "yairaike"),
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
    for (latn, _, kana, _, _, _) in &TEST_CASES {
        assert_eq!(convert_latn_to_kana(latn), kana.to_owned());
    }
}

#[test]
fn test_convert_kana_to_latn() {
    for (_, _, kana, _, _, latn) in &TEST_CASES {
        assert_eq!(convert_kana_to_latn(kana), latn.to_owned());
    }
}

#[test]
fn test_convert_latn_to_cyrl() {
    for (latn, _, _, cyrl, _, _) in &TEST_CASES {
        assert_eq!(convert_latn_to_cyrl(latn), cyrl.to_owned());
    }
}

#[test]
fn test_convert_cyrl_to_latn() {
    for (latn, _, _, cyrl, _, _) in &TEST_CASES {
        assert_eq!(convert_cyrl_to_latn(cyrl), latn.to_owned());
    }
}
