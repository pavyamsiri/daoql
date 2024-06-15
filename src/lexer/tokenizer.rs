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

        if let Some(kind) = self.lex_inline_comment() {
            return Some(Token {
                kind,
                span: Span {
                    start: current_offset,
                    length: self.offset - current_offset,
                },
            });
        }

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

        // Handle string literals
        if let Some(kind) = self.lex_string_literal() {
            return Some(Token {
                kind,
                span: Span {
                    start: current_offset,
                    length: self.offset - current_offset,
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
            ';' => TokenKind::Semicolon,
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
            "CREATE" => Some(TokenKind::Keyword(Keyword::Create)),
            "TABLE" => Some(TokenKind::Keyword(Keyword::Table)),
            _ => Some(TokenKind::Identifier),
        }
    }

    fn lex_string_literal(&mut self) -> Option<TokenKind> {
        let current_char = self.peek()?;
        let (open_quote, kind) = match current_char {
            '"' => ('"', TokenKind::DoubleQuotedStringLiteral),
            '\'' => ('\'', TokenKind::SingleQuotedStringLiteral),
            _ => return None,
        };
        self.advance();
        while let Some(current_char) = self.peek() {
            self.advance();
            if current_char == open_quote {
                break;
            }
        }
        Some(kind)
    }

    fn lex_inline_comment(&mut self) -> Option<TokenKind> {
        let current_char = self.peek()?;
        let next_char = self.peek_ahead(1)?;
        match (current_char, next_char) {
            ('-', '-') => {
                self.advance();
                self.advance();
            }
            _ => return None,
        }

        while let Some(current_char) = self.peek() {
            if current_char == '\r' || current_char == '\n' {
                break;
            }
            self.advance();
        }

        Some(TokenKind::InlineComment)
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
