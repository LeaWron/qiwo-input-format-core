pub(crate) fn is_han(ch: char) -> bool {
    matches!(
        ch as u32,
        0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xF900..=0xFAFF
            | 0x20000..=0x2A6DF
            | 0x2A700..=0x2B73F
            | 0x2B740..=0x2B81F
            | 0x2B820..=0x2CEAF
            | 0x2CEB0..=0x2EBEF
            | 0x30000..=0x3134F
    )
}

pub(crate) fn is_ascii_alnum(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
}

pub(crate) fn is_ascii_punctuation(ch: char) -> bool {
    ch.is_ascii_punctuation()
}

pub(crate) fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_supported_characters() {
        assert!(is_han('中'));
        assert!(is_han('𠀀'));
        assert!(is_ascii_alnum('A'));
        assert!(is_ascii_alnum('7'));
        assert!(is_ascii_punctuation(','));
        assert!(is_whitespace(' '));
    }

    #[test]
    fn excludes_out_of_scope_characters() {
        assert!(!is_han('あ'));
        assert!(!is_han('한'));
        assert!(!is_han('😀'));
        assert!(!is_ascii_alnum('Ａ'));
        assert!(!is_ascii_punctuation('，'));
    }
}
