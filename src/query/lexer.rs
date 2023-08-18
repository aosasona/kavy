// Thanks to Thorsten Ball for the lexer implementation in his book "Writing an Interpreter in Go"

/*
Example usage:

SET key, value
GET key
DEL key
CLEAR ALL
*/

use crate::query::token::Token;

pub struct Lexer {
    input: Vec<u8>,
    char: u8,
    current_position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            char: 0,
            current_position: 0,
            read_position: 0,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::EOF || token == Token::Illegal {
                break;
            }

            tokens.push(token);
        }

        return tokens;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.char.to_ascii_lowercase() {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => {
                let identifier = self.read_identifier();

                return match identifier.to_lowercase().as_str() {
                    "set" => Token::Set,
                    "get" => Token::Get,
                    "del" => Token::Del,
                    "flush" => Token::Flush,
                    _ => Token::Identifier(identifier),
                };
            }
            b'=' => Token::Equals,
            b';' => Token::SemiColon,
            0 => Token::EOF,
            _ => Token::Illegal,
        };

        self.read_char();

        return token;
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position;

        while self.char.is_ascii_alphanumeric() {
            self.read_char()
        }

        return String::from_utf8_lossy(&self.input[start_pos..self.current_position]).to_string();
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            match self.input.get(self.read_position) {
                Some(ch) => self.char = *ch,
                None => self.char = 0,
            }

            self.current_position = self.read_position;
            self.read_position += 1;
        }
    }
}
