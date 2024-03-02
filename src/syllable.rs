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
    let mut syllable_map: HashMap<usize, usize> = HashMap::new();
    let mut syllable_count = 1;

    for (i, char) in latn.chars().enumerate() {
        if VOWELS.contains(char) {
            if i > 0 && CONSONANTS.contains(latn.chars().nth(i - 1).unwrap()) {
                syllable_map.insert(i - 1, syllable_count);
            }
            syllable_map.insert(i, syllable_count);
            syllable_count += 1;
        }
    }

    // Fill codas
    for i in 0..latn.len() {
        if !syllable_map.contains_key(&i) {
            syllable_map.insert(i, *syllable_map.get(&(i - 1)).unwrap_or(&0));
        }
    }

    // Group and extract syllables
    let mut syllables: Vec<String> = Vec::new();
    let mut current_group_id = 1;
    let mut head = 0;
    let chars: Vec<char> = latn.chars().collect();

    for i in 0..latn.len() {
        if *syllable_map.get(&i).unwrap_or(&0) != current_group_id {
            current_group_id = *syllable_map.get(&i).unwrap_or(&0);
            syllables.push(chars[head..i].iter().collect());
            head = i;
        }
    }

    syllables.push(chars[head..].iter().collect());

    // Remove apostrophes from syllables
    syllables.iter().map(|s| s.replace("'", "")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate() {
        let separated = separate("eyaykosiramsuypa");
        assert_eq!(separated, vec!["e", "yay", "ko", "si", "ram", "suy", "pa"]);
    }
}
