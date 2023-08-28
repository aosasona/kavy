use super::{
    token::Token,
    Lexer, {Command, Op},
};

use super::{get_syntax, Identifier};

pub struct Parser {
    input: Vec<u8>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            input: input.into_bytes(),
        }
    }

    pub fn parse(&mut self) -> Result<Command, String> {
        let mut lexer = Lexer::new(self.input.clone());
        let tokens = lexer.lex();
        if tokens.len() == 0 {
            return Err("no query provided, see the readme for more info".to_string());
        }

        let op = match tokens.get(0).expect("expected a token") {
            Token::Set => Op::Set,
            Token::Get => Op::Get,
            Token::Del => Op::Del,
            Token::Flush => Op::Flush,
            _ => {
                return Err(
                    format!("`{}` is not a valid keyword!", tokens[0].to_string()).to_string(),
                )
            }
        };

        let syntax = get_syntax(&op);

        // check that the query ends with a semicolon
        if tokens[tokens.len() - 1] != Token::SemiColon {
            return Err("expected a semicolon at the end of the query".to_string());
        }

        let mut cmd = Command::new(op);

        for (i, token) in tokens.iter().enumerate() {
            let expected_token = &syntax.expected_tokens_pattern[i];

            // if it is not an identifier, then we want to check that it is the expected token
            if *token != *expected_token
                && !(*expected_token == Token::AnyIdentifier && token.is_identifier())
            {
                return Err(format!(
                    "expected `{}`, but got `{}`",
                    expected_token.name(),
                    token.to_string()
                ));
            }

            // if it is the first identifier we encounter, then we want to set it as the key
            if token.is_identifier() && cmd.key.is_none() {
                cmd.set_key(token.to_string());
            }

            // if it is the first identifier we encounter after an =, then we know that it is the value
            if token.is_identifier() && cmd.value.is_none() && i > 0 {
                match tokens.get(i - 1) {
                    Some(prev_token) => {
                        if *prev_token == Token::Equals {
                            cmd.set_value(token.to_string());
                        }
                    }
                    None => {}
                }
            }
        }

        // check that the command is valid for the given operation
        self.is_valid_command(&cmd)?;

        Ok(cmd)
    }

    fn is_valid_command(&self, cmd: &Command) -> Result<(), String> {
        match cmd.op {
            Op::Set => {
                if cmd.key.is_none() && cmd.value.is_none() {
                    return Err("expected a key and a value".to_string());
                } else if cmd.key.is_none() {
                    return Err("expected a key".to_string());
                } else if cmd.value.is_none() {
                    return Err("expected a value".to_string());
                }
            }
            Op::Get => {
                if cmd.key.is_none() {
                    return Err("expected a key".to_string());
                }
            }
            Op::Del => {
                if cmd.key.is_none() {
                    return Err("expected a key".to_string());
                }
            }
            Op::Flush => {}
        }

        Ok(())
    }
}
