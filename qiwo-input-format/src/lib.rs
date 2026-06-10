//! Shared commit-text formatter for Qiwo frontends.

mod char_class;
mod spacing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatOptions {
    pub auto_spacing_enabled: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            auto_spacing_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommitContext<'a> {
    pub before_cursor: Option<&'a str>,
    pub after_cursor: Option<&'a str>,
}

pub fn format_commit_text(
    commit_text: &str,
    context: CommitContext<'_>,
    options: FormatOptions,
) -> String {
    if !options.auto_spacing_enabled {
        return commit_text.to_owned();
    }

    spacing::format_with_context(commit_text, context.before_cursor, context.after_cursor)
}
