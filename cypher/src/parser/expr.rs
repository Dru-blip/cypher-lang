use crate::lexer::token::Token;
use display_json::DisplayAsJsonPretty;
use serde::Serialize;

#[derive(Debug, Serialize, DisplayAsJsonPretty)]
pub struct Program {
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { body: statements }
    }
}

#[derive(Debug, Serialize, DisplayAsJsonPretty,Clone)]
pub enum Statement {
    ExpressionStatement {
        expr: Expression,
    },
    PrintStatement {
        expr: Expression,
    },
    VariableStatement {
        ident: Token,
        expr: Option<Box<Statement>>,
    },
    IFStatement {
        condition: Expression,
        then: Box<Statement>,
        _else: Option<Box<Statement>>,
    },
    BlockStatement {
        statements: Vec<Statement>,
    },
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    ForStatement {
        initializer: Box<Statement>,
        condition: Expression,
        increment: Expression,
        body: Box<Statement>,
    },
    FunctionDeclaration {
        name: Token,
        args: Vec<Token>,
        body: Box<Statement>,
    },
    ReturnStatement {
        expr: Option<Expression>,
    },
}

#[derive(Debug, Serialize, DisplayAsJsonPretty,Clone)]
pub enum Expression {
    VariableAssignment {
        identifier: Token,
        expr: Box<Expression>,
    },
    GroupingExpression {
        exp: Box<Expression>,
    },
    UnaryExpression {
        op: Token,
        rhs: Box<Expression>,
    },
    BinaryExpression {
        lhs: Box<Expression>,
        op: Token,
        rhs: Box<Expression>,
    },
    IncrementDecrement {
        op: Token,
        identifier: Token,
    },
    ArrayDeclaration{
        elements:Vec<Expression>
    },
    ArrayIndexing{
        ident:Box<Expression>,
        index:Box<Expression>
    },
    FunctionCall {
        calle: Box<Expression>,
        args: Vec<Expression>,
    },
    GetExpression{
        identifier:Token,
        exp:Box<Expression>
    },
    Literal {
        value: Token,
    },
}
