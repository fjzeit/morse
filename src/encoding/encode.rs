use super::morse;

fn get_morse_index(c: char) -> Option<usize> {
    let uc = c.to_ascii_uppercase() as u8;
    match uc {
        b'A'..=b'Z' => Some((uc - b'A') as usize),
        b'0'..=b'9' => Some((uc - b'0' + 26) as usize),
        _ => morse::MORSE_TABLE
            .iter()
            .skip(36)
            .position(|&(ch, _)| ch == c)
            .map(|pos| pos + 36),
    }
}

fn morse_length(text: &str) -> usize {
    let mut length = str::len(text);
    for (_, b) in text.chars().enumerate() {
        if let Some(pos) = get_morse_index(b) {
            length += morse::MORSE_TABLE[pos].1.len();
        }
    }

    length
}

pub fn to_morse(text: &str) -> Result<String, String> {
    let morse_len = morse_length(text);
    let mut morse: Vec<u8> = Vec::with_capacity(morse_len);

    for b in text.chars() {
        if let Some(pos) = get_morse_index(b) {
            let i = morse::MORSE_TABLE[pos];
            morse.extend_from_slice(i.1.as_bytes());
            morse.extend(b" ");
        } else {
            return Err(format!("Unsupported character: {}", b));
        }
    }

    Ok(String::from_utf8(morse).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest] // ensure that get_morse_index returns correct index for a given character
    fn test_get_morse_index() {
        for (c, i) in morse::MORSE_TABLE.iter().enumerate() {
            let idx = get_morse_index(i.0).unwrap();
            assert_eq!(idx, c);
            assert_eq!(morse::MORSE_TABLE[idx], *i);
        }
    }

    #[rstest]
    #[case("e", ". ")]
    #[case("E", ". ")]
    #[case("hello", ".... . .-.. .-.. --- ")]
    #[case("HELLO", ".... . .-.. .-.. --- ")]
    #[case("HeLlO", ".... . .-.. .-.. --- ")]
    #[case("HeLlO WoRlD", ".... . .-.. .-.. --- / .-- --- .-. .-.. -.. ")]
    fn test_to_morse(#[case] input: &str, #[case] expected: &str) {
        // test the encoding
        let actual = to_morse(input).unwrap();
        assert_eq!(expected, actual);
        // make sure the underlying vec wasn't resized
        let expected_len = morse_length(input);
        assert_eq!(expected_len, actual.len());
    }
}