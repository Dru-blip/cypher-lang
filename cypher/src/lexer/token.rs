use serde::{Serialize};
use display_json::DisplayAsJsonPretty;


#[derive(Debug, PartialEq, Clone, Copy, Eq,Serialize,DisplayAsJsonPretty)]
pub enum TokenType {
    Nil,
    Number,
    String,

    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Not,
    Bang,

    Increment,
    Decrement,

    SemiColon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    RSqBracket,
    LSqBracket,
    Dot,



    And,
    Or,
    Assign,

    While,
    For,
    Do,
    Break,
    Continue,
    Until,
    Return,
    If,
    Elseif,
    Else,
    Repeat,
    In,
    Function,
    Print,
    Let,
    Goto,
    True,
    False,

    Identifier,

    Eof
}

#[derive(Debug, Clone, Copy,DisplayAsJsonPretty,Serialize)]
pub struct Location {
    pub line: u32,
    pub col: u32,
    pub index: usize,
}


impl Location {
    pub fn new(line: u32, col: u32, index: usize) -> Self {
        Self { line, col, index }
    }
}

#[derive(Debug,DisplayAsJsonPretty,Serialize)]
pub struct TokenList{
    pub tokens: Vec<Token>,
}

impl TokenList {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}


#[derive(Debug,DisplayAsJsonPretty,Serialize)]
pub struct Token {
    pub kind: TokenType,
    location: Location,
    pub value: Option<String>,
}


impl Clone for Token{
    fn clone(&self) -> Self {
        Self{
            kind:self.kind,
            location:self.location,
            value:self.value.clone()
        }
    }
}


impl Token {
    pub fn new(kind:TokenType,location:Location,value:Option<String>) ->Token{
        Self{
            kind,
            location,
            value
        }
    }
    pub fn get_location(&self)->Location {
        self.location
    }
}