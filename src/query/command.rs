use crate::query::token::Token;

pub enum Op {
    Set,
    Get,
    Del,
    Flush,
}

pub trait Command {
    fn get_op(&self) -> Op;
    fn get_key(&self) -> Option<String>;
    fn get_value(&self) -> Option<String>;
}

pub struct Syntax {
    op: Op,
    expected_tokens_pattern: Vec<Token>,
}

pub fn get_command(op: Op) -> Syntax {
    return match op {
        Op::Set => Syntax {
            op: Op::Set,
            expected_tokens_pattern: vec![
                Token::Set,
                Token::AnyIdentifier,
                Token::Equals,
                Token::AnyIdentifier,
                Token::SemiColon,
            ],
        },
        Op::Get => Syntax {
            op: Op::Get,
            expected_tokens_pattern: vec![Token::Get, Token::AnyIdentifier, Token::SemiColon],
        },
        Op::Del => Syntax {
            op: Op::Del,
            expected_tokens_pattern: vec![Token::Del, Token::AnyIdentifier, Token::SemiColon],
        },
        Op::Flush => Syntax {
            op: Op::Flush,
            expected_tokens_pattern: vec![Token::Flush, Token::SemiColon],
        },
    };
}
