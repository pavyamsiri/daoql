mod keyword;
pub use keyword::Keyword;

#[derive(Debug)]
pub enum TokenKind {
    Comma,
    Semicolon,

    Keyword(Keyword),
    Identifier,

    SingleQuotedStringLiteral,
    DoubleQuotedStringLiteral,

    InlineComment,
}
