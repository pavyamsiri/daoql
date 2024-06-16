use std::ops::Range;

use super::kind::TokenKind;

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub length: usize,
}

impl Span {
    pub const fn end(&self) -> usize {
        self.start + self.length
    }

    pub const fn range(&self) -> Range<usize> {
        self.start..self.end()
    }
}

#[derive(Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub fn begin_highlight(&self) -> &'static str {
        match self.kind {
            TokenKind::Asterisk => "\x1b[33m",                    // Yellow
            TokenKind::Comma => "\x1b[33m",                       // Yellow
            TokenKind::Colon => "\x1b[33m",                       // Yellow
            TokenKind::Semicolon => "\x1b[33m",                   // Yellow
            TokenKind::LeftParenthesis => "\x1b[33m",             // Yellow
            TokenKind::RightParenthesis => "\x1b[33m",            // Yellow
            TokenKind::Percent => "\x1b[33m",                     // Yellow
            TokenKind::Period => "\x1b[33m",                      // Yellow
            TokenKind::Plus => "\x1b[33m",                        // Yellow
            TokenKind::Minus => "\x1b[33m",                       // Yellow
            TokenKind::Solidus => "\x1b[33m",                     // Yellow
            TokenKind::LessThan => "\x1b[33m",                    // Yellow
            TokenKind::GreaterThan => "\x1b[33m",                 // Yellow
            TokenKind::Equals => "\x1b[33m",                      // Yellow
            TokenKind::Underscore => "\x1b[33m",                  // Yellow
            TokenKind::Ampersand => "\x1b[33m",                   // Yellow
            TokenKind::QuestionMark => "\x1b[33m",                // Yellow
            TokenKind::Circumflex => "\x1b[33m",                  // Yellow
            TokenKind::Tilde => "\x1b[33m",                       // Yellow
            TokenKind::VerticalBar => "\x1b[33m",                 // Yellow
            TokenKind::Keyword(_) => "\x1b[1;31m",                // Bright Red
            TokenKind::Identifier => "\x1b[1;34m",                // Bright Blue
            TokenKind::SingleQuotedStringLiteral => "\x1b[1;35m", // Magenta
            TokenKind::DoubleQuotedStringLiteral => "\x1b[1;35m", // Magenta
            TokenKind::IntegerLiteral => "\x1b[1;36m",            // Cyan
            TokenKind::DecimalLiteral => "\x1b[1;36m",            // Cyan
            TokenKind::HexadecimalLiteral => "\x1b[1;36m",        // Cyan
            TokenKind::ExponentLiteral => "\x1b[1;36m",           // Cyan
            TokenKind::InlineComment => "\x1b[90m",               // Bright Black (Gray)
        }
    }

    pub fn end_highlight(&self) -> &'static str {
        "\x1b[0m"
    }
}
