use super::kind::TokenKind;

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}
