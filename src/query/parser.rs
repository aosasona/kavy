use crate::query::lexer::Lexer;
use crate::query::{
    command::{Command, Op},
    token::Token,
};

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

        let op = match tokens[0] {
            Token::Set => Op::Set,
            Token::Get => Op::Get,
            Token::Del => Op::Del,
            Token::Flush => Op::Flush,
            _ => {
                return Err(
                    format!("`{}` is not a valid command!", tokens[0].to_string()).to_string(),
                )
            }
        };

        let cmd = Command::new(op);
        Ok(cmd)
    }
}
