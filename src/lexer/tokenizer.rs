use super::kind::{Keyword, TokenKind};
use super::token::{Span, Token};

pub struct Lexer<'a> {
    source: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, offset: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let current_offset = self.offset;

        // Handle single character tokens
        if let Some(kind) = self.lex_single_character_token() {
            return Some(Token {
                kind,
                span: Span {
                    start: current_offset,
                    length: 1,
                },
            });
        }

        // Handle identifiers or keywords
        if let Some(kind) = self.lex_identifier_or_keyword() {
            return Some(Token {
                kind,
                span: Span {
                    start: current_offset,
                    length: self.offset - current_offset,
                },
            });
        }

        None
    }

    fn lex_single_character_token(&mut self) -> Option<TokenKind> {
        let current_char = self.peek()?;
        let kind = match current_char {
            ',' => TokenKind::Comma,
            _ => return None,
        };

        self.advance();
        Some(kind)
    }

    fn lex_identifier_or_keyword(&mut self) -> Option<TokenKind> {
        let current_offset = self.offset;
        while let Some(current_char) = self.peek() {
            if current_char.is_alphanumeric() || current_char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let token_slice = &self.source[current_offset..self.offset];
        if token_slice.is_empty() {
            return None;
        }

        match token_slice.to_uppercase().as_str() {
            "SELECT" => Some(TokenKind::Keyword(Keyword::Select)),
            "FROM" => Some(TokenKind::Keyword(Keyword::From)),
            _ => Some(TokenKind::Identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.offset)
    }

    fn peek_ahead(&self, next: usize) -> Option<char> {
        self.source.chars().nth(self.offset + next)
    }
}
