use std::fmt::Display;

pub trait Identifier {
    fn is_identifier(&self) -> bool;
}

#[derive(PartialEq)]
pub enum Token {
    Set,
    Get,
    Del,
    Flush,

    Equals,
    SemiColon,

    #[allow(dead_code)]
    AnyIdentifier, // this is a special token that matches any identifier, used for the parser

    Identifier(String),
    Illegal,
    EOF,
}

impl Identifier for Token {
    fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            _ => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            Token::Set => "SET",
            Token::Get => "GET",
            Token::Del => "DEL",
            Token::Flush => "FLUSH",
            Token::Equals => "=",
            Token::SemiColon => ";",
            Token::AnyIdentifier => "ANY_IDENTIFIER",
            Token::Identifier(value) => {
                return write!(f, "{}", value);
            }
            Token::Illegal => "ILLEGAL",
            Token::EOF => "EOF",
        };
        write!(f, "{}", token)
    }
}
