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

        // Handle comments
        if let Some(token) = self.lex_inline_comment() {
            return Some(token);
        }

        // Handle integer literals
        if let Some(token) = self.lex_integer_literal() {
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
        let kind = match current_char {
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LeftParenthesis,
            ')' => TokenKind::RightParenthesis,
            _ => return None,
        };

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

        let kind = match token_slice.to_uppercase().as_str() {
            "SELECT" => TokenKind::Keyword(Keyword::Select),
            "FROM" => TokenKind::Keyword(Keyword::From),
            "CREATE" => TokenKind::Keyword(Keyword::Create),
            "TABLE" => TokenKind::Keyword(Keyword::Table),
            "INT" => TokenKind::Keyword(Keyword::Int),
            "PRIMARY" => TokenKind::Keyword(Keyword::Primary),
            "KEY" => TokenKind::Keyword(Keyword::Key),
            "VARCHAR" => TokenKind::Keyword(Keyword::VarChar),
            "DATE" => TokenKind::Keyword(Keyword::Date),
            "DECIMAL" => TokenKind::Keyword(Keyword::Decimal),
            "INSERT" => TokenKind::Keyword(Keyword::Insert),
            "INTO" => TokenKind::Keyword(Keyword::Into),
            "VALUE" => TokenKind::Keyword(Keyword::Value),
            "VALUES" => TokenKind::Keyword(Keyword::Values),
            _ => TokenKind::Identifier,
        };

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

    fn lex_integer_literal(&mut self) -> Option<Token> {
        // TODO: Handle hexadecimal
        let initial_offset = self.offset;
        let current_char = self.peek()?;
        if !current_char.is_ascii_digit() {
            return None;
        }

        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        Some(Token {
            kind: TokenKind::IntegerLiteral,
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
