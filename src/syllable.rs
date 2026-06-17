use crate::conversion::latin::{CONSONANTS, VOWELS};
use std::collections::HashMap;

/// Syllabify an Ainu word
///
/// Divide a romanized Ainu word into syllables.
/// # Arguments
/// * `latn` - A string slice that holds the romanized Ainu word.
///  
/// # Example
/// ```
/// use ainconv::separate;
/// let separated = separate("eyaykosiramsuypa"); // ey-ay-ko-si-ram-suy-pa
/// println!("{:?}", separated); // ["pro", "gram", "ming", "is", "fun"]
/// ```
pub fn separate(latn: &str) -> Vec<String> {
    // Index by CHARACTER throughout (not byte), so multi-byte characters such as
    // the glottal stop ’ (U+2019) are handled correctly rather than corrupting the
    // syllable map (and, downstream, panicking on a non-char-boundary slice).
    let chars: Vec<char> = latn.chars().collect();
    let n = chars.len();
    let mut syllable_map: HashMap<usize, usize> = HashMap::new();
    let mut syllable_count = 1;

    for i in 0..n {
        if VOWELS.contains(chars[i]) {
            if i > 0 && CONSONANTS.contains(chars[i - 1]) {
                syllable_map.insert(i - 1, syllable_count);
            }
            syllable_map.insert(i, syllable_count);
            syllable_count += 1;
        }
    }

    // Fill codas: an unassigned character joins the syllable on its left.
    for i in 0..n {
        if !syllable_map.contains_key(&i) {
            let prev = i
                .checked_sub(1)
                .and_then(|p| syllable_map.get(&p))
                .copied()
                .unwrap_or(0);
            syllable_map.insert(i, prev);
        }
    }

    // Group and extract syllables.
    let mut syllables: Vec<String> = Vec::new();
    let mut current_group_id = 1;
    let mut head = 0;
    for i in 0..n {
        if *syllable_map.get(&i).unwrap_or(&0) != current_group_id {
            current_group_id = *syllable_map.get(&i).unwrap_or(&0);
            syllables.push(chars[head..i].iter().collect());
            head = i;
        }
    }
    syllables.push(chars[head..].iter().collect());

    // Remove apostrophes from syllables.
    syllables.iter().map(|s| s.replace('\'', "")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate() {
        let separated = separate("eyaykosiramsuypa");
        assert_eq!(separated, vec!["e", "yay", "ko", "si", "ram", "suy", "pa"]);
    }

    #[test]
    fn separate_handles_glottal_stop() {
        // The multi-byte glottal stop ’ (U+2019) must be indexed by character;
        // mixing byte and char indices used to corrupt the map and could panic.
        assert_eq!(separate("ne\u{2019}"), vec!["ne\u{2019}"]);
        assert_eq!(separate("a\u{2019}e"), vec!["a", "\u{2019}e"]);
    }

    #[test]
    fn separate_bare_consonant_does_not_panic() {
        // The coda-fill loop computed `i - 1` at `i == 0` (usize underflow).
        let _ = separate("k");
        let _ = separate("");
    }
}
