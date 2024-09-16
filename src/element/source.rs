use std::borrow::Cow;

/// Source of text and its properties for text operations.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// Plain text.
    ///
    /// Has the same formatting for all text.
    Plain(Cow<'static, str>),
}

impl From<Cow<'static, str>> for Source {
    fn from(cow: Cow<'static, str>) -> Self {
        Self::Plain(cow)
    }
}

impl From<&'static str> for Source {
    fn from(str: &'static str) -> Self {
        Self::Plain(Cow::Borrowed(str))
    }
}

impl From<String> for Source {
    fn from(string: String) -> Self {
        Self::Plain(Cow::Owned(string))
    }
}
