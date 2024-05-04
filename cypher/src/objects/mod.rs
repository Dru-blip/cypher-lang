use std::fmt::Display;

use display_json::DisplayAsJsonPretty;
use serde::Serialize;

use crate::{lexer::token::Token, parser::expr::Statement};

type BuiltinFunction = fn(args: Vec<Object>) -> Object;

#[derive(Debug, Clone)]
pub struct BuiltinFn {
    pub Fn: BuiltinFunction,
}

#[derive(Debug, Clone, Serialize, DisplayAsJsonPretty)]
pub struct Function {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: Box<Statement>,
    // scope:SymbolTable
}

impl BuiltinFn {
    pub fn new(function: BuiltinFunction) -> Self {
        Self { Fn: function }
    }
}

impl Function {
    pub fn new(name: Token, args: Vec<Token>, body: Box<Statement>) -> Self {
        Self { name, args, body }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Error(String),
    Func(Function),
    Tuple(Box<Object>,Box<Object>),
    Array(Vec<Object>),
    // Property(Box<Object>),
    Builtin(BuiltinFn),
}

impl Object {
    
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(a) => write!(f, "{}", a),
            Object::Str(a) => write!(f, "{}", a),
            Object::Bool(a) => write!(f, "{}", a),
            Object::Nil => write!(f, "nil"),
            Object::Error(message) => write!(f, "{}", message),
            Object::Func(func) => {
                write!(f, "function({})", &func.name.value.as_ref().unwrap())
            }
            Object::Builtin(fun) => {
                write!(f,"native fn<{:?}>",&fun.Fn)
            },
            Object::Tuple(_, _) => todo!(),
            Object::Array(array) => {
                let _=write!(f,"[");
                for obj in array{
                    let _=write!(f,"{},",obj);
                }
                write!(f,"]")
            },
        }
    }
}
