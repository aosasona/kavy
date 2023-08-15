pub mod token;
// Example:
//
// SET key, value AS STRING
// GET key => will automatically get as the type it was set as
// DEL key
// CLEAR ALL
//

pub enum TokenKind {
    // commands
    Set,
    Get,
    Del,
    As,
    ClearAll,

    // data types
    String,
    Number,
    Bool,

    // other
    Identifier,
    Illegal,
    EOF,
}

struct Token {
    kind: TokenKind,
    literal: String,
}
