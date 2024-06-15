mod keyword;
pub use keyword::Keyword;

#[derive(Debug)]
pub enum TokenKind {
    Comma,

    Keyword(Keyword),
    Identifier,
}
