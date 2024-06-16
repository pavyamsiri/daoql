mod keyword;
pub use keyword::Keyword;

#[derive(Debug)]
pub enum TokenKind {
    Asterisk,
    Comma,
    Colon,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    Percent,
    Period,
    Plus,
    Minus,
    Solidus,
    LessThan,
    GreaterThan,
    Equals,
    Underscore,
    Ampersand,
    QuestionMark,
    Circumflex,
    Tilde,
    VerticalBar,

    Keyword(Keyword),
    Identifier,

    SingleQuotedStringLiteral,
    DoubleQuotedStringLiteral,

    IntegerLiteral,
    DecimalLiteral,
    HexadecimalLiteral,
    ExponentLiteral,

    InlineComment,
}

impl TokenKind {
    pub fn parse_single_character(c: char) -> Option<TokenKind> {
        match c {
            '*' => Some(TokenKind::Asterisk),
            ',' => Some(TokenKind::Comma),
            ':' => Some(TokenKind::Colon),
            ';' => Some(TokenKind::Semicolon),
            '(' => Some(TokenKind::LeftParenthesis),
            ')' => Some(TokenKind::RightParenthesis),
            '%' => Some(TokenKind::Percent),
            '.' => Some(TokenKind::Period),
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '/' => Some(TokenKind::Solidus),
            '<' => Some(TokenKind::LessThan),
            '>' => Some(TokenKind::GreaterThan),
            '=' => Some(TokenKind::Equals),
            '_' => Some(TokenKind::Underscore),
            '&' => Some(TokenKind::Ampersand),
            '?' => Some(TokenKind::QuestionMark),
            '^' => Some(TokenKind::Circumflex),
            '~' => Some(TokenKind::Tilde),
            '|' => Some(TokenKind::VerticalBar),
            _ => None,
        }
    }
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
