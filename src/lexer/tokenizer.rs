use super::token::Token;

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
        None
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
