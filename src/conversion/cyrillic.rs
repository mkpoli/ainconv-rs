/// Convert romanized Ainu to Cyrillic
///
/// # Arguments
///
/// * `latn` - A string slice that holds the romanized Ainu word.
///
/// # Returns
///
/// * `String` - The Cyrillic representation of the input string.
///
/// # Example
///
/// ```
/// use ainconv::convert_latn_to_cyrl;
/// let cyrl = convert_latn_to_cyrl("aynu");
/// assert_eq!(cyrl, "айну");
/// ```
pub fn convert_latn_to_cyrl(latn: &str) -> String {
    let mut result = String::new();
    let mut chars = latn.chars().peekable();

    while let Some(current_char) = chars.next() {
        let current_lower = current_char.to_lowercase().next().unwrap();

        let cyrl: Option<&str> = match chars.peek() {
            Some(&next_char) if current_lower == 'y' && "uaoe".contains(next_char) => {
                let next_lower = next_char.to_lowercase().next().unwrap();
                // Consume the next character
                chars.next();
                match (current_lower, next_lower) {
                    ('y', 'u') => Some("ю"),
                    ('y', 'a') => Some("я"),
                    ('y', 'o') => Some("ё"),
                    ('y', 'e') => Some("е"),
                    _ => unreachable!(),
                }
            }
            _ => match current_lower {
                'a' => Some("а"),
                'i' => Some("и"),
                'u' => Some("у"),
                'e' => Some("э"),
                'o' => Some("о"),
                'k' => Some("к"),
                's' => Some("с"),
                't' => Some("т"),
                'c' => Some("ц"),
                'h' => Some("х"),
                'm' => Some("м"),
                'n' => Some("н"),
                'p' => Some("п"),
                'r' => Some("р"),
                'w' => Some("в"),
                'y' => Some("й"),
                '\'' => Some("ъ"),
                '’' => Some(""),
                _ => None,
            },
        };

        // If the original character was uppercase, convert the result to uppercase
        let cyrl_result = match cyrl {
            Some(c) => {
                if current_char.is_uppercase() {
                    c.chars().flat_map(char::to_uppercase).collect::<String>()
                } else {
                    c.to_string()
                }
            }
            None => current_char.to_string(),
        };
        result.push_str(&cyrl_result);
    }

    result
}

/// Convert Cyrillic to romanized Ainu
///
/// # Arguments
///
/// * `cyrl` - A string slice that holds the Cyrillic word.
///
/// # Returns
///
/// * `String` - The romanized Ainu representation of the input string.
///
/// # Example
///
/// ```
/// use ainconv::convert_cyrl_to_latn;
/// let latn = convert_cyrl_to_latn("айну");
/// assert_eq!(latn, "aynu");
/// ```
pub fn convert_cyrl_to_latn(cyrl: &str) -> String {
    let mut result = String::new();
    let mut chars = cyrl.chars().peekable();

    while let Some(current_char) = chars.next() {
        let current_lower = current_char.to_lowercase().next().unwrap();

        let cyrl: Option<&str> = match chars.peek() {
            Some(&next_char) if current_lower == 'й' && "уаоэ".contains(next_char) => {
                let next_lower = next_char.to_lowercase().next().unwrap();
                // Consume the next character
                chars.next();
                match next_lower {
                    'у' => Some("y’u"),
                    'а' => Some("y’a"),
                    'о' => Some("y’o"),
                    'э' => Some("y’e"),
                    _ => unreachable!(),
                }
            }
            _ => match current_lower {
                'ю' => Some("yu"),
                'я' => Some("ya"),
                'ё' => Some("yo"),
                'е' => Some("ye"),
                'а' => Some("a"),
                'и' => Some("i"),
                'у' => Some("u"),
                'э' => Some("e"),
                'о' => Some("o"),
                'к' => Some("k"),
                'с' => Some("s"),
                'т' => Some("t"),
                'ц' => Some("c"),
                'х' => Some("h"),
                'м' => Some("m"),
                'н' => Some("n"),
                'п' => Some("p"),
                'р' => Some("r"),
                'в' => Some("w"),
                'й' => Some("y"),
                'ъ' => Some("'"),
                'ь' => None,
                '’' => None,
                ' ' => None,
                _ => None,
            },
        };

        // If the original character was uppercase, convert the result to uppercase
        let cyrl_result = match cyrl {
            Some(c) => {
                if current_char.is_uppercase() {
                    c.chars().flat_map(char::to_uppercase).collect::<String>()
                } else {
                    c.to_string()
                }
            }
            None => current_char.to_string(),
        };
        result.push_str(&cyrl_result);
    }
    result
}
