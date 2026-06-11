use crate::char_class::{
    is_ascii_alnum, is_ascii_punctuation, is_han, is_line_break, is_whitespace,
};

pub(crate) fn format_internal(commit_text: &str) -> String {
    let mut output = String::with_capacity(commit_text.len());
    let mut previous = None;

    for current in commit_text.chars() {
        if let Some(left) = previous
            && need_space_between(left, current)
            && !output.ends_with(char::is_whitespace)
        {
            output.push(' ');
        }

        output.push(current);
        previous = Some(current);
    }

    output
}

pub(crate) fn format_with_context(
    commit_text: &str,
    before_cursor: Option<&str>,
    after_cursor: Option<&str>,
) -> String {
    let mut output = format_internal(commit_text);

    if output.is_empty() {
        return output;
    }

    if let Some(before) = before_cursor
        && let (Some(left), Some(right)) = (before_cursor_boundary(before), output.chars().next())
        && need_space_between(left, right)
    {
        output.insert(0, ' ');
    }

    if let Some(after) = after_cursor
        && let (Some(left), Some(right)) =
            (output.chars().next_back(), after_cursor_boundary(after))
        && need_space_between(left, right)
    {
        output.push(' ');
    }

    output
}

pub(crate) fn need_space_between(left: char, right: char) -> bool {
    if is_whitespace(left) || is_whitespace(right) {
        return false;
    }

    (is_han(left) && is_ascii_alnum(right))
        || (is_ascii_alnum(left) && is_han(right))
        || (is_ascii_punctuation(left) && is_han(right))
}

fn before_cursor_boundary(before_cursor: &str) -> Option<char> {
    let boundary = before_cursor.chars().next_back()?;
    (!is_line_break(boundary)).then_some(boundary)
}

fn after_cursor_boundary(after_cursor: &str) -> Option<char> {
    let boundary = after_cursor.chars().next()?;
    (!is_line_break(boundary)).then_some(boundary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_internal_boundaries() {
        assert_eq!(format_internal("中文ABC123测试"), "中文 ABC123 测试");
        assert_eq!(format_internal(",中文测试,"), ", 中文测试,");
    }

    #[test]
    fn avoids_duplicate_spaces_and_excluded_classes() {
        assert_eq!(format_internal("中文 ABC"), "中文 ABC");
        assert_eq!(format_internal("ABC123"), "ABC123");
        assert_eq!(format_internal("あ中文"), "あ中文");
        assert_eq!(format_internal("中文Ａ"), "中文Ａ");
    }

    #[test]
    fn formats_boundaries_when_surrounding_context_is_available() {
        assert_eq!(format_with_context("中文", Some("abc"), None), " 中文");
        assert_eq!(format_with_context("abc", Some("中文"), None), " abc");
        assert_eq!(format_with_context("中文", Some(","), None), " 中文");
        assert_eq!(format_with_context("中文", None, Some("abc")), "中文 ");
        assert_eq!(format_with_context(",", None, Some("中文")), ", ");
    }

    #[test]
    fn treats_line_breaks_as_hard_context_boundaries() {
        assert_eq!(format_internal("中文\nabc"), "中文\nabc");
        assert_eq!(format_internal("中文\r\n123"), "中文\r\n123");
        assert_eq!(format_with_context("abc", Some("中文\n"), None), "abc");
        assert_eq!(format_with_context("123", Some("中文\r\n"), None), "123");
        assert_eq!(format_with_context("中文", None, Some("\nabc")), "中文");
        assert_eq!(format_with_context("中文", None, Some("\r\n123")), "中文");
    }

    #[test]
    fn leaves_empty_commit_text_empty_with_context() {
        assert_eq!(format_with_context("", Some("abc"), Some("中文")), "");
    }
}
