use std::char;

static FIRST_CHAR: usize = 33;
static LAST_CHAR: usize = 126;
static RANGE_MOD: usize = LAST_CHAR - FIRST_CHAR + 1;

static FIRST_CHAR2: usize = 192;
static LAST_CHAR2: usize = 250;
static SUM2: usize = FIRST_CHAR2 + LAST_CHAR2;

fn f(x: usize) -> usize {
    4 * x * x - 3 * x + 2
}

fn g(x: usize) -> usize {
    (x * 3 / 2) + 1
}

pub fn custom_vigenere(text: &str, key: &str, encrypt: bool) -> String {
    let mut key_char_code = 0;
    let mut change_at = 1;
    let key_chars: Vec<char> = key.chars().collect();
    let mut j = 0;

    text.chars()
        .map(|c| {
            let char_code = c as usize;

            if char_code >= FIRST_CHAR && char_code <= LAST_CHAR2 {
                j += 1;
            }

            if j == change_at {
                let this_key_char_code = key_chars[j % key.len()] as usize;
                key_char_code = (key_char_code + f(j) * this_key_char_code) % RANGE_MOD;
                change_at = g(change_at)
            }

            let result;
            if char_code >= FIRST_CHAR && char_code <= LAST_CHAR {
                if encrypt {
                    result = FIRST_CHAR + (char_code + key_char_code - FIRST_CHAR) % RANGE_MOD;
                } else {
                    result = FIRST_CHAR
                        + (char_code + RANGE_MOD - FIRST_CHAR - key_char_code) % RANGE_MOD;
                }
            } else if char_code >= FIRST_CHAR2 && char_code <= LAST_CHAR2 {
                result = SUM2 - char_code;
            } else {
                return c;
            };

            match u8::try_from(result) {
                Ok(r) => r as char,
                Err(_) => c,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_one(original: &str) {
        let key = "lorem ipsum";

        let encrypted = custom_vigenere(original, key, true);
        assert_ne!(&encrypted, original);

        let decrypted = custom_vigenere(&encrypted, key, false);
        assert_eq!(original, &decrypted);
    }

    #[test]
    fn basic() {
        test_one("hello world");
    }

    #[test]
    fn with_128_plus() {
        test_one("{ hello á world ó '}");
    }

    #[test]
    fn with_emoji() {
        test_one("😎 hello ✅ world");
    }
}
