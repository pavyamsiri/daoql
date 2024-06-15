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

    InlineComment,
}
