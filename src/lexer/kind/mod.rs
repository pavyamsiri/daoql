mod keyword;
pub use keyword::Keyword;

#[derive(Debug)]
pub enum TokenKind {
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,

    Keyword(Keyword),
    Identifier,

    SingleQuotedStringLiteral,
    DoubleQuotedStringLiteral,

    IntegerLiteral,

    InlineComment,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "Keyword(\x1b[36m{keyword:?}\x1b[0m)"),
            TokenKind::Identifier => write!(f, "\x1b[31m{self:?}\x1b[0m"),
            _ => write!(f, "{self:?}"),
        }
    }
}
