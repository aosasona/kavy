use crate::query::Token;

pub enum Op {
    Set,
    Get,
    Del,
    Flush,
}

pub struct Command {
    pub op: Op,

    // depending on the operation, these fields may or may not be populated, for example, if
    // the operation is `get`, then `key` will be populated, but `value` will not be
    // populated
    pub key: Option<String>,
    pub value: Option<String>,
}

impl Command {
    pub fn new(op: Op) -> Command {
        Command {
            op,
            key: None,
            value: None,
        }
    }

    pub fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }
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
