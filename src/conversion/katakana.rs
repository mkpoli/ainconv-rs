// See notice in the TypeScript version https://github.com/mkpoli/ainconv/blob/main/src/conversion/katakana.ts
use crate::conversion::latin::CONSONANTS;
use crate::syllable::separate;
use crate::util::remove_acute_accent;

/// Convert romanized Ainu to Katakana
///
/// # Arguments
///
/// * `latn` - A string slice that holds the romanized Ainu word.
///
/// # Returns
///
/// * `String` - The Katakana representation of the input string.
///
/// # Example
///
/// ```
/// use ainconv::convert_latn_to_kana;
/// let kana = convert_latn_to_kana("aynu");
/// assert_eq!(kana, "アイヌ");
/// ```
pub fn convert_latn_to_kana(latn: &str) -> String {
    let latn = latn.replace("=", "");
    let latn = remove_acute_accent(&latn);

    let syllables = separate(&latn);

    let mut result = String::new();

    for syllable in syllables.iter() {
        // println!("syllable {}", syllable);
        if syllable.len() == 0 {
            continue;
        }

        // V => (V, _)
        // VC => (V, C)
        // CVC => (CV, C)

        let last_char = syllable.chars().last().unwrap();

        let (remains, coda) = if CONSONANTS.contains(last_char) {
            let (remains, coda) = syllable.split_at(syllable.len() - 1);
            (remains, coda)
        } else {
            (syllable.as_str(), "")
        };

        let converted_remains = match remains {
            "a" => "ア",
            "i" => "イ",
            "u" => "ウ",
            "e" => "エ",
            "o" => "オ",
            "'a" => "ア",
            "'i" => "イ",
            "'u" => "ウ",
            "'e" => "エ",
            "'o" => "オ",
            "’a" => "ア",
            "’i" => "イ",
            "’u" => "ウ",
            "’e" => "エ",
            "’o" => "オ",
            "ka" => "カ",
            "ki" => "キ",
            "ku" => "ク",
            "ke" => "ケ",
            "ko" => "コ",
            "sa" => "サ",
            "si" => "シ",
            "su" => "ス",
            "se" => "セ",
            "so" => "ソ",
            "ta" => "タ",
            "tu" => "ト゚",
            "te" => "テ",
            "to" => "ト",
            "ca" => "チャ",
            "ci" => "チ",
            "cu" => "チュ",
            "ce" => "チェ",
            "co" => "チョ",
            "na" => "ナ",
            "ni" => "ニ",
            "nu" => "ヌ",
            "ne" => "ネ",
            "no" => "ノ",
            "ha" => "ハ",
            "hi" => "ヒ",
            "hu" => "フ",
            "he" => "ヘ",
            "ho" => "ホ",
            "pa" => "パ",
            "pi" => "ピ",
            "pu" => "プ",
            "pe" => "ペ",
            "po" => "ポ",
            "ma" => "マ",
            "mi" => "ミ",
            "mu" => "ム",
            "me" => "メ",
            "mo" => "モ",
            "ya" => "ヤ",
            "yi" => "イ",
            "yu" => "ユ",
            "ye" => "イェ",
            "yo" => "ヨ",
            "ra" => "ラ",
            "ri" => "リ",
            "ru" => "ル",
            "re" => "レ",
            "ro" => "ロ",
            "wa" => "ワ",
            "wi" => "ヰ",
            "we" => "ヱ",
            "wo" => "ヲ",
            "nn" => "ン",
            "tt" => "ッ",
            _ => syllable,
        };
        result.push_str(converted_remains);

        let vowel = remains.chars().last();

        let converted_coda = {
            match coda {
                "w" => "ゥ",
                "y" => "ィ",
                "m" => "ㇺ",
                "n" => "ㇴ",
                "s" => "ㇱ",
                "p" => "ㇷ゚",
                "t" => "ッ",
                "T" => "ㇳ",
                "k" => "ㇰ",
                "r" => match vowel {
                    Some('a') => "ㇻ",
                    Some('i') => "ㇼ",
                    Some('u') => "ㇽ",
                    Some('e') => "ㇾ",
                    Some('o') => "ㇿ",
                    _ => "ㇽ",
                },
                "h" => match vowel {
                    Some('a') => "ㇵ",
                    Some('i') => "ㇶ",
                    Some('u') => "ㇷ",
                    Some('e') => "ㇸ",
                    Some('o') => "ㇹ",
                    _ => "ㇷ",
                },
                "x" => match vowel {
                    Some('a') => "ㇵ",
                    Some('i') => "ㇶ",
                    Some('u') => "ㇷ",
                    Some('e') => "ㇸ",
                    Some('o') => "ㇹ",
                    _ => "ㇷ",
                },
                _ => coda,
            }
        };
        result.push_str(converted_coda);
    }

    result
        .replace('ィ', "イ")
        .replace('ゥ', "ウ")
        .replace("ㇴ", "ン")
        .replace("ヱ", "ウェ")
        .replace("ヰ", "ウィ")
        .replace("ヲ", "ウォ")
}

/// Convert Katakana to romanized Ainu
///
/// # Arguments
///
/// * `kana` - A string slice that holds the Katakana word.
///
/// # Returns
///
/// * `String` - The romanized Ainu representation of the input string.
///
/// # Example
///
/// ```
/// use ainconv::convert_kana_to_latn;
/// let latn = convert_kana_to_latn("アイヌ");
/// assert_eq!(latn, "ainu");
/// ```
pub fn convert_kana_to_latn(kana: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut chars = kana.chars().peekable();
    while let Some(current_char) = chars.next() {
        let next_char = chars.peek();

        let converted_digraph: Option<&str> = match (current_char, next_char) {
            // ('ア', Some('イ')) => Some("ay"),
            // ('ア', Some('ウ')) => Some("aw"),
            // ('ア', Some('エ')) => Some("ay"),
            ('イ', Some('ェ')) => Some("ye"),
            ('ウ', Some('ェ')) => Some("we"),
            ('ウ', Some('ィ')) => Some("wi"),
            ('ウ', Some('ォ')) => Some("wo"),
            ('ト', Some('ゥ')) => Some("tu"),
            // ('エ', Some('イ')) => Some("ey"),
            // ('オ', Some('イ')) => Some("oy"),
            // ('ウ', Some('イ')) => Some("uy"),
            ('ㇷ', Some('゚')) => Some("p"),
            ('ト', Some('゚')) => Some("tu"),
            ('チ', Some('ャ')) => Some("ca"),
            ('チ', Some('ュ')) => Some("cu"),
            ('チ', Some('ェ')) => Some("ce"),
            ('チ', Some('ョ')) => Some("co"),
            ('オ', Some('イ')) => Some("oy"),
            ('エ', Some('イ')) => Some("ey"),
            ('ウ', Some('イ')) => Some("uy"),
            _ => None,
        };

        if let Some(digraph) = converted_digraph {
            result.push(digraph.to_owned());
            chars.next();
            continue;
        }

        let converted = match current_char {
            'ア' => Some("a"),
            'イ' => Some("i"),
            'ウ' => Some("u"),
            'エ' => Some("e"),
            'オ' => Some("o"),
            'カ' => Some("ka"),
            'キ' => Some("ki"),
            'ク' => Some("ku"),
            'ケ' => Some("ke"),
            'コ' => Some("ko"),
            'サ' => Some("sa"),
            'シ' => Some("si"),
            'ス' => Some("su"),
            'セ' => Some("se"),
            'ソ' => Some("so"),
            'タ' => Some("ta"),
            'チ' => Some("ci"),
            'テ' => Some("te"),
            'ト' => Some("to"),
            'ナ' => Some("na"),
            'ニ' => Some("ni"),
            'ヌ' => Some("nu"),
            'ネ' => Some("ne"),
            'ノ' => Some("no"),
            'ハ' => Some("ha"),
            'ヒ' => Some("hi"),
            'フ' => Some("hu"),
            'ヘ' => Some("he"),
            'ホ' => Some("ho"),
            'パ' => Some("pa"),
            'ピ' => Some("pi"),
            'プ' => Some("pu"),
            'ペ' => Some("pe"),
            'ポ' => Some("po"),
            'マ' => Some("ma"),
            'ミ' => Some("mi"),
            'ム' => Some("mu"),
            'メ' => Some("me"),
            'モ' => Some("mo"),
            'ヤ' => Some("ya"),
            'ユ' => Some("yu"),
            'ヨ' => Some("yo"),
            'ラ' => Some("ra"),
            'リ' => Some("ri"),
            'ル' => Some("ru"),
            'レ' => Some("re"),
            'ロ' => Some("ro"),
            'ワ' => Some("wa"),
            'ヲ' => Some("wo"),
            'ン' => Some("n"),
            'ㇺ' => Some("m"),
            'ㇴ' => Some("n"),
            'ゥ' => Some("w"),
            'ィ' => Some("y"),
            'ㇷ' => Some("h"),
            'ㇱ' => Some("s"),
            'ッ' => Some("t"),
            'ㇳ' => Some("t"),
            'ㇰ' => Some("k"),
            'ㇵ' => Some("x"),
            'ㇶ' => Some("x"),
            'ㇸ' => Some("x"),
            'ㇹ' => Some("x"),
            'ァ' => Some("a"),
            'ェ' => Some("e"),
            'ォ' => Some("o"),
            'ㇻ' => Some("r"),
            'ㇼ' => Some("r"),
            'ㇽ' => Some("r"),
            'ㇾ' => Some("r"),
            'ㇿ' => Some("r"),
            _ => None,
        };
        match converted {
            Some(c) => result.push(c.to_owned()),
            None => {
                result.push(current_char.to_string());
            }
        }
    }

    let joined = result.join("’");
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    // let joined = result.replace("'", "’");
    let mut final_result = Vec::new();

    for (i, char) in joined.chars().enumerate() {
        if char == '’' {
            if i > 0 && is_vowel(joined.chars().nth(i - 1).unwrap()) {
                // If the previous character is a vowel, remove the apostrophe
                continue;
            }
            if i < joined.len() - 1 && !is_vowel(joined.chars().nth(i + 1).unwrap()) {
                // If the next character is not a vowel, remove the apostrophe
                continue;
            }
        }
        final_result.push(char);
    }

    final_result.iter().collect()
}
