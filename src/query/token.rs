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
    Illegal(u8),
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
            Token::Illegal(value) => {
                return write!(f, "{}", value);
            }
            Token::EOF => "EOF",
        };
        write!(f, "{}", token)
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Set => "SET".to_string(),
            Token::Get => "GET".to_string(),
            Token::Del => "DEL".to_string(),
            Token::Flush => "FLUSH".to_string(),
            Token::Equals => "=".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::AnyIdentifier => "ANY_IDENTIFIER".to_string(),
            Token::Identifier(value) => value.to_string(),
            Token::Illegal(value) => format!("{}", *value as char),
            Token::EOF => "EOF".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Token::Set => "SET".to_string(),
            Token::Get => "GET".to_string(),
            Token::Del => "DEL".to_string(),
            Token::Flush => "FLUSH".to_string(),
            Token::Equals => "=".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::AnyIdentifier => "IDENTIFIER".to_string(),
            Token::Identifier(_) => "IDENTIFIER".to_string(),
            Token::Illegal(_) => "ILLEGAL".to_string(),
            Token::EOF => "EOF".to_string(),
        }
    }
}
