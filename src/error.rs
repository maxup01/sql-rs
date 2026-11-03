#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnbalancedParentheses,
    UnexpectedChar,
    UnexpectedEnd,
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}
