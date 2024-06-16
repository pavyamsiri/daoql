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

    pub fn peek_next(&mut self) -> Option<Token> {
        let old_offset = self.offset;
        let next_token = self.next_token();
        self.offset = old_offset;
        next_token
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        // Handle comments
        if let Some(token) = self.lex_inline_comment() {
            return Some(token);
        }

        // Handle numeric literals
        if let Some(token) = self.lex_numeric_literal() {
            return Some(token);
        }

        // Handle single character tokens
        if let Some(token) = self.lex_single_character_token() {
            return Some(token);
        }

        // Handle string literals
        if let Some(token) = self.lex_string_literal() {
            return Some(token);
        }

        // Handle identifiers or keywords
        if let Some(token) = self.lex_identifier_or_keyword() {
            return Some(token);
        }

        None
    }

    fn lex_single_character_token(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
        let current_char = self.peek()?;
        let kind = TokenKind::parse_single_character(current_char)?;

        self.advance();
        Some(Token {
            kind,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
    }

    fn lex_identifier_or_keyword(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
        let first_char = self.peek()?;
        if !first_char.is_ascii_alphabetic() {
            return None;
        }
        self.advance();
        while let Some(current_char) = self.peek() {
            if current_char.is_ascii_alphanumeric() || current_char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let token_slice = &self.source[initial_offset..self.offset];
        if token_slice.is_empty() {
            return None;
        }

        let kind = Keyword::parse(token_slice.to_uppercase().as_str())
            .map_or(TokenKind::Identifier, TokenKind::Keyword);

        Some(Token {
            kind,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
    }

    fn lex_string_literal(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
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
        Some(Token {
            kind,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
    }

    fn lex_inline_comment(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
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

        Some(Token {
            kind: TokenKind::InlineComment,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
    }

    fn lex_numeric_literal(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
        let first_char = self.peek()?;
        let second_char = self.peek_ahead(1)?;

        // Hexadecimal
        if let ('0', 'x') = (first_char, second_char) {
            return self.lex_hexadecimal_literal();
        }

        if !first_char.is_ascii_digit() {
            return None;
        }

        let mut kind = TokenKind::IntegerLiteral;
        while let Some(c) = self.peek() {
            if c == '.' {
                kind = TokenKind::DecimalLiteral;
                self.advance();
                continue;
            } else if c == 'e' || c == 'E' {
                kind = TokenKind::ExponentLiteral;
                self.advance();
                continue;
            }

            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        Some(Token {
            kind,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
    }

    fn lex_hexadecimal_literal(&mut self) -> Option<Token> {
        let initial_offset = self.offset;
        let first_char = self.peek()?;
        let second_char = self.peek_ahead(1)?;
        match (first_char, second_char) {
            ('0', 'x') => {}
            _ => return None,
        }

        self.advance();
        self.advance();

        while let Some(c) = self.peek() {
            if !c.is_digit(16) {
                break;
            }
            self.advance();
        }

        Some(Token {
            kind: TokenKind::HexadecimalLiteral,
            span: Span {
                start: initial_offset,
                length: self.offset - initial_offset,
            },
        })
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
