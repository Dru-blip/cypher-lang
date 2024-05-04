use std::vec;

use crate::{
    errors::syntax::SyntaxError,
    lexer::{
        scanner::Scanner,
        token::{Token, TokenType},
    },
};

use super::expr::{Expression, Program, Statement};

pub struct Parser<'a> {
    lexer: &'a mut Scanner<'a>,
    file_name: &'a String,
    current_token: Option<Token>,
    next_token: Option<Token>,
    lines: &'a Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(
        lexer: &'a mut Scanner<'a>,
        file_name: &'a String,
        lines: &'a Vec<&'a str>,
    ) -> Parser<'a> {
        Self {
            lexer,
            file_name,
            current_token: None,
            next_token: None,
            lines,
        }
    }

    fn advance(&mut self) {
        match self.lexer.next_token() {
            Ok(token) => {
                self.current_token = self.next_token.clone();
                self.next_token = Some(token);
            }
            Err(err) => {
                println!("{}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn set_tokens(&mut self) {
        self.advance();
    }

    fn check_token(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.current_token.as_ref() {
            if token.kind == token_type {
                return true;
            }
        }
        false
    }
    fn check_next_token(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.next_token.as_ref() {
            if token.kind == token_type {
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType) {
        while let Some(token) = self.current_token.as_ref() {
            if token.kind == token_type {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn generate_syntax_error(&self, message: String) -> Result<Expression, SyntaxError> {
        let location = self.lexer.get_location();
        let line = self.lines[(location.line - 1) as usize];

        Err(SyntaxError::new(
            self.file_name.clone(),
            location.line,
            location.col,
            message,
            line.to_owned(),
        ))
    }

    fn generate_syntax_error_for_statements(
        &self,
        message: String,
    ) -> Result<Statement, SyntaxError> {
        let location = self.current_token.as_ref().unwrap().get_location();
        let line = self.lines[location.line as usize - 1];
        Err(SyntaxError::new(
            self.file_name.clone(),
            location.line,
            location.col,
            message,
            line.to_owned(),
        ))
    }

    pub fn generate_json(statements: Vec<Statement>) -> String {
        let mut json = String::new();
        for statement in statements {
            json.push_str(statement.to_string().as_str());
        }
        // serde_json::to_string(&statements).unwrap()
        json
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = vec![];
        self.advance();
        self.advance();
        loop {
            if let Some(token) = self.current_token.clone() {
                if token.kind == TokenType::Eof {
                    break;
                }
                let statement = self.declaration();
                if statement.is_ok() {
                    statements.push(statement.unwrap());
                } else {
                    let err = statement.err();
                    println!("{}", err.unwrap());
                    std::process::exit(1);
                }
            }
        }

        Program::new(statements)
    }

    fn declaration(&mut self) -> Result<Statement, SyntaxError> {
        /*

         declaration    → varDecl | statement |function-declaration;

        */

        match self.current_token.clone() {
            Some(token) => {
                if self.check_token(TokenType::Let) {
                    self.parse_variable_declaration()
                } else if self.check_token(TokenType::Function) {
                    self.parse_function_declaration()
                } else {
                    self.parse_statements()
                }
            }
            None => self.generate_syntax_error_for_statements("Eof parsing".to_string()),
        }
    }

    fn parse_statements(&mut self) -> Result<Statement, SyntaxError> {
        match self.current_token.clone() {
            Some(token) => match token.kind {
                TokenType::If => self.parse_if_statment(),
                TokenType::Print => self.print_statement(),
                TokenType::Let => self.parse_variable_declaration(),
                TokenType::While => self.parse_while_statements(),
                TokenType::For => self.parse_for_statements(),
                TokenType::Return => self.parse_return_statement(),
                TokenType::LBrace => self.parse_block(),
                _ => self.expression_statement(),
            },
            None => self.generate_syntax_error_for_statements("Eof parsing error".to_string()),
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.advance();
        let expr = self.parse_expression();

        if expr.is_err() {
            println!("{}", expr.unwrap());
            std::process::exit(1);
        }

        Ok(Statement::ReturnStatement {
            expr: Some(expr.unwrap()),
        })
    }

    fn parse_function_args(&mut self) -> Vec<Token> {
        let mut args: Vec<Token> = vec![];

        while !self.check_token(TokenType::RParen) {
            if self.check_token(TokenType::Comma) {
                self.advance();
                continue;
            }
            args.push(self.current_token.clone().unwrap());
            self.advance();
        }
        args
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, SyntaxError> {
        /*
          function Declaration -> "function" Identifier "(" <arguments> "}" <block>
          arguments -> identifier ("," identifier)*
        */
        self.advance();
        match self.current_token.clone() {
            Some(token) => {
                if !self.check_token(TokenType::Identifier) {
                    return self
                        .generate_syntax_error_for_statements("expected identifier".to_owned());
                }
                let ident = token;
                let mut args: Vec<Token> = vec![];
                self.advance();
                if self.check_token(TokenType::LParen) {
                    self.advance();

                    loop {
                        if self.check_token(TokenType::RParen) {
                            break;
                        }
                        if self.check_token(TokenType::LBrace) {
                            return self
                                .generate_syntax_error_for_statements("expected )".to_string());
                        }
                        if self.check_token(TokenType::Comma) {
                            self.advance();
                            continue;
                        }
                        args.push(self.current_token.clone().unwrap());
                        self.advance();
                    }
                    args = self.parse_function_args();

                    self.advance();

                    if self.check_token(TokenType::LBrace) {
                        let body = self.parse_block();

                        if body.is_err() {
                            println!("{}", body.err().unwrap());
                            std::process::exit(1)
                        }

                        return Ok(Statement::FunctionDeclaration {
                            name: ident,
                            args,
                            body: Box::new(body.unwrap()),
                        });
                    } else {
                        self.generate_syntax_error_for_statements("expected {".to_string())
                    }
                } else {
                    self.generate_syntax_error_for_statements("expected (".to_string())
                }
            }
            None => self.generate_syntax_error_for_statements("".to_string()),
        }
    }

    fn parse_for_statements(&mut self) -> Result<Statement, SyntaxError> {
        self.advance();
        match self.current_token.clone() {
            Some(token) => {
                let mut increment: Option<Expression> = None;
                let mut initializer: Option<Statement> = None;
                if self.check_token(TokenType::Let) {
                    match self.parse_variable_declaration() {
                        Ok(statement) => {
                            initializer = Some(statement);
                        }
                        Err(err) => {
                            println!("{}", err);
                            std::process::exit(1)
                        }
                    }
                } else {
                    return self.generate_syntax_error_for_statements(
                        "loop variable should be initialized".to_owned(),
                    );
                }

                self.consume(TokenType::SemiColon);

                let end = self.parse_expression();

                if end.is_err() {
                    println!("{}", end.unwrap());
                    std::process::exit(1)
                }

                if self.check_token(TokenType::SemiColon) {
                    self.advance();

                    let expr = self.parse_expression();
                    if expr.is_ok() {
                        increment = Some(expr.unwrap());
                    } else {
                        println!("{}", expr.unwrap());
                    }
                }

                if self.check_token(TokenType::LBrace) {
                    let body = self.parse_block();

                    Ok(Statement::ForStatement {
                        initializer: Box::new(initializer.unwrap()),
                        condition: end.unwrap(),
                        increment: increment.unwrap(),
                        body: Box::new(body.unwrap()),
                    })
                } else {
                    self.generate_syntax_error_for_statements("".to_string())
                }
            }
            None => self.generate_syntax_error_for_statements("".to_string()),
        }
    }

    fn parse_while_statements(&mut self) -> Result<Statement, SyntaxError> {
        self.advance();
        match self.current_token.clone() {
            Some(token) => {
                if self.check_token(TokenType::LBrace) {
                    return self
                        .generate_syntax_error_for_statements("expected condition".to_owned());
                }
                let expr = self.parse_expression();

                if self.check_token(TokenType::LBrace) {
                    let body = self.parse_block();

                    Ok(Statement::WhileStatement {
                        condition: expr.unwrap(),
                        body: Box::new(body.unwrap()),
                    })
                } else {
                    self.generate_syntax_error_for_statements("expected do keyword".to_string())
                }
            }
            None => self.generate_syntax_error_for_statements("error".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Statement, SyntaxError> {
        self.advance();
        let mut statements: Vec<Statement> = vec![];

        while !self.check_token(TokenType::RBrace) {
            let stmt = self.declaration();
            if stmt.is_ok() {
                statements.push(stmt.unwrap());
            }
        }

        if self.check_token(TokenType::RBrace) {
            self.advance();
            return Ok(Statement::BlockStatement {
                statements: statements,
            });
        }

        self.generate_syntax_error_for_statements("expected }".to_string())
    }

    fn parse_if_statment(&mut self) -> Result<Statement, SyntaxError> {
        /*
          ifStatement → "if" <expression> "{" <statement>*  ("else" <statement>*)? "}" ;
        */

        match self.current_token.clone() {
            Some(token) => {
                self.advance();
                if self.check_token(TokenType::LBrace) {
                    return self
                        .generate_syntax_error_for_statements("expected condition".to_owned());
                }
                let expr = self.parse_expression();
                // self.advance();
                if expr.is_err() {
                    return self.generate_syntax_error_for_statements(expr.unwrap().to_string());
                }
                let mut else_statement: Option<Statement> = None;
                let mut then_statement: Option<Statement> = None;

                if self.check_token(TokenType::LBrace) {
                    let stmt = self.parse_block();
                    if stmt.is_err() {
                        return stmt;
                    }

                    then_statement = Some(stmt.unwrap());
                }

                // self.advance();

                if self.check_token(TokenType::Else) {
                    self.advance();
                    if self.check_token(TokenType::LBrace) {
                        let stmt = self.parse_block();
                        if stmt.is_ok() {
                            else_statement = Some(stmt.unwrap())
                        }
                    }
                    // self.advance();
                }

                if else_statement.is_some() {
                    return Ok(Statement::IFStatement {
                        condition: expr.unwrap(),
                        then: Box::new(then_statement.unwrap()),
                        _else: Some(Box::new(else_statement.unwrap())),
                    });
                }

                Ok(Statement::IFStatement {
                    condition: expr.unwrap(),
                    then: Box::new(then_statement.unwrap()),
                    _else: None,
                })
            }
            None => self.generate_syntax_error_for_statements("end of file error".to_string()),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, SyntaxError> {
        /*
         varDecl → let IDENTIFIER ( "=" expression )? ";" ;
        */
        self.advance();
        match self.current_token.clone() {
            Some(token) => {
                let identifier = token;
                self.advance();

                let mut expr: Option<Box<Statement>> = None;
                if self.check_token(TokenType::Assign) {
                    self.advance();
                    if self.check_token(TokenType::Function) {
                        self.advance();
                        if self.check_token(TokenType::LParen) {
                            self.advance();
                            let args = self.parse_function_args();
                            self.advance();
                            if !self.check_token(TokenType::LBrace) {
                                return self.generate_syntax_error_for_statements(
                                    "Expected Function Body".to_owned(),
                                );
                            }
                            let function_body = self.parse_block();
                            if function_body.is_err() {
                                return function_body;
                            }
                            return Ok(Statement::FunctionDeclaration {
                                name: identifier,
                                args,
                                body: Box::new(function_body.unwrap()),
                            });
                        }
                    } else {
                        let exp = self.parse_expression();
                        if exp.is_ok() {
                            expr = Some(Box::new(Statement::ExpressionStatement {
                                expr: exp.unwrap(),
                            }))
                        } else {
                            let err = self.generate_syntax_error_for_statements(
                                "expected assignment expression".to_string(),
                            );
                            println!("{}", err.unwrap());
                            std::process::exit(1);
                        }
                    }
                }

                Ok(Statement::VariableStatement {
                    ident: identifier,
                    expr,
                })
            }
            None => self.generate_syntax_error_for_statements("Eof parsing error".to_string()),
        }
    }

    fn print_statement(&mut self) -> Result<Statement, SyntaxError> {
        self.advance();
        match self.current_token.clone() {
            Some(token) => {
                if token.kind == TokenType::LParen {
                    let expr = self.parse_expression();
                    if expr.is_ok() {
                        return Ok(Statement::PrintStatement {
                            expr: expr.unwrap(),
                        });
                    } else {
                        return self.generate_syntax_error_for_statements(
                            "expected 1 or more arguments,found 0".to_string(),
                        );
                    }
                }
                println!("unexpected token");
                std::process::exit(1)
            }
            None => self.generate_syntax_error_for_statements("end of file".to_string()),
        }
    }

    fn expression_statement(&mut self) -> Result<Statement, SyntaxError> {
        let expr = self.parse_expression();
        if expr.is_ok() {
            return Ok(Statement::ExpressionStatement {
                expr: expr.unwrap(),
            });
        }

        self.generate_syntax_error_for_statements("expected expression".to_owned())
    }

    pub fn parse_expression(&mut self) -> Result<Expression, SyntaxError> {
        if self.check_token(TokenType::LSqBracket){
            let mut elements: Vec<Expression> =vec![];
            self.advance();
            while !self.check_token(TokenType::RSqBracket) {
                if self.check_token(TokenType::Comma){
                    self.advance();
                    continue;
                }
                let element=self.parse_expression();
                if element.is_err() {
                    return element;
                }
                elements.push(element.unwrap())
            }
            self.advance();
           return  Ok(Expression::ArrayDeclaration { elements })
        }
        self.parse_variable_reassignment()
    }

    fn parse_variable_reassignment(&mut self) -> Result<Expression, SyntaxError> {
        if self.check_token(TokenType::Identifier) {
            match self.current_token.clone() {
                Some(token) => {
                    if self.check_next_token(TokenType::Assign) {
                        let ident = token;
                        self.advance();
                        self.advance();
                        let expr = self.parse_variable_reassignment();
                        if expr.is_ok() {
                            return Ok(Expression::VariableAssignment {
                                identifier: ident,
                                expr: Box::new(expr.unwrap()),
                            });
                        } else {
                            println!("{}", expr.unwrap());
                            std::process::exit(1)
                        }
                    } else {
                        return self.equality();
                    }
                }
                _ => return self.generate_syntax_error("eof error".to_string()),
            }
        }
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, SyntaxError> {
        let mut lhs = self.parse_logical_and();
        while self.check_token(TokenType::Or) {
            let op = self.current_token.take().unwrap();
            self.advance();
            let rhs = self.parse_logical_and();
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }
        lhs
    }

    fn parse_logical_and(&mut self) -> Result<Expression, SyntaxError> {
        let mut lhs = self.equality();
        while self.check_token(TokenType::And) {
            let op = self.current_token.take().unwrap();
            self.advance();
            let rhs = self.equality();
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }

        lhs
    }

    fn equality(&mut self) -> Result<Expression, SyntaxError> {
        /*
        equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        */
        let mut lhs = self.comparison();

        while self.check_token(TokenType::NotEqual) || self.check_token(TokenType::Equal) {
            let op = self.current_token.take().unwrap();
            self.advance();
            let rhs = self.comparison();
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }

        lhs
    }

    fn comparison(&mut self) -> Result<Expression, SyntaxError> {
        /*
          comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        */
        let mut lhs = self.term();

        while self.check_token(TokenType::GreaterThan)
            || self.check_token(TokenType::GreaterThanOrEqual)
            || self.check_token(TokenType::LessThan)
            || self.check_token(TokenType::LessThanOrEqual)
        {
            let op = self.current_token.take().unwrap();
            self.advance();
            let rhs = self.term();
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }

        lhs
    }

    fn term(&mut self) -> Result<Expression, SyntaxError> {
        /*
        term → factor ( ( "-" | "+" ) factor )* ;
         */

        let mut lhs = self.factor();

        while self.check_token(TokenType::Minus) || self.check_token(TokenType::Plus) {
            let op = self.current_token.take().unwrap();
            self.advance();
            let rhs = self.factor();
            // if(rhs.is_err()){
            //     return self.generate_syntax_error("expression expected".to_owned());
            // }
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }

        lhs
    }

    fn factor(&mut self) -> Result<Expression, SyntaxError> {
        /*
        factor  → unary ( ( "/" | "*" ) unary )* ;
         */
        let mut lhs = self.unary();

        while self.check_token(TokenType::Modulo)
            || self.check_token(TokenType::Slash)
            || self.check_token(TokenType::Star)
        {
            let op = self.current_token.clone().unwrap();
            self.advance();
            let rhs = self.unary();
            lhs = Ok(Expression::BinaryExpression {
                lhs: Box::new(lhs.unwrap()),
                op,
                rhs: Box::new(rhs.unwrap()),
            })
        }

        lhs
    }

    fn unary(&mut self) -> Result<Expression, SyntaxError> {
        match self.current_token.clone() {
            Some(token) => {
                if self.check_token(TokenType::Not) || self.check_token(TokenType::Minus) {
                    let op = token;
                    self.advance();
                    let expr = self.unary();
                    if expr.is_ok() {
                        Ok(Expression::UnaryExpression {
                            op: op.clone(),
                            rhs: Box::new(expr.unwrap()),
                        })
                    } else {
                        println!("{}", expr.unwrap());
                        std::process::exit(1);
                    }
                } else {
                    let expr = self.parse_increment_decrement();
                    if expr.is_ok() {
                        return expr;
                    } else {
                        println!("{}", expr.unwrap());
                        std::process::exit(1);
                    }
                }
            }
            None => self.generate_syntax_error("Eof parsing error".to_string()),
        }
    }

    fn parse_increment_decrement(&mut self) -> Result<Expression, SyntaxError> {
        match self.current_token.clone() {
            Some(token) => {
                if self.check_token(TokenType::Identifier)
                    && (self.check_next_token(TokenType::Increment)
                        || self.check_next_token(TokenType::Decrement))
                {
                    let ident = token;
                    self.advance();

                    let operator = self.current_token.clone().unwrap();
                    self.advance();
                    return Ok(Expression::IncrementDecrement {
                        op: operator,
                        identifier: ident,
                    });
                } else {
                    let expr = self.parse_array_index();

                    if expr.is_err() {
                        let err = expr.err().unwrap();
                        println!("{}", err);
                        std::process::exit(1)
                    }

                    expr
                }
            }
            None => self.generate_syntax_error("Eof parsing error".to_string()),
        }
    }

    fn parse_array_index(&mut self) ->Result<Expression,SyntaxError>{
        let expr=self.parse_property_access();
        if expr.is_err() {
            return expr;
        }

        if self.check_token(TokenType::LSqBracket){
            self.advance();
            let index=self.parse_property_access();
            if index.is_err(){
                return index;
            }
            self.advance();
            return Ok(Expression::ArrayIndexing { ident: Box::new(expr.unwrap()), index: Box::new(index.unwrap()) })
        }
        expr
    }

    fn parse_property_access(&mut self) -> Result<Expression, SyntaxError> {
        let current_token = self.current_token.clone();
        let mut expr = self.primary();
        // self.advance();
        loop {
            if self.check_token(TokenType::LParen) {
                self.advance();
                let mut args: Vec<Expression> = vec![];
                while !self.check_token(TokenType::RParen) {
                    if self.check_token(TokenType::Comma) {
                        self.advance();
                    }
                    let expression = self.parse_expression();
                    if expression.is_err() {
                        println!("{}", expression.unwrap());
                        std::process::exit(1);
                    }

                    args.push(expression.unwrap());
                }
                self.advance();
                expr=Ok(Expression::FunctionCall {
                    calle: Box::new(expr.unwrap()),
                    args,
                });
            }
            if self.check_token(TokenType::Dot) {
                self.advance();
                
                let mut ident: Option<&Token> =None;
                if self.check_token(TokenType::Identifier){
                    ident=self.current_token.as_ref();
                }
                
                if ident.is_none(){
                    return self.generate_syntax_error("expected identifier after .".to_owned())
                }
                
                expr= Ok(Expression::GetExpression {
                    identifier: ident.unwrap().clone(),
                    exp: Box::new(expr.unwrap()),
                });

                self.advance();
            } else {
                break;
            }
        }
        expr
    }

    // fn parse_function_call(&mut self) -> Result<Expression, SyntaxError> {
    //     if self.check_token(TokenType::Identifier) && self.check_next_token(TokenType::LParen) {
    //         let expr = self.current_token.clone().unwrap();
    //         self.advance();
    //         if self.check_token(TokenType::LParen) {
    //             self.advance();
    //             let mut args: Vec<Expression> = vec![];
    //             while !self.check_token(TokenType::RParen) {
    //                 if self.check_token(TokenType::Comma) {
    //                     self.advance();
    //                 }
    //                 let expression = self.parse_expression();
    //                 if expression.is_err() {
    //                     println!("{}", expression.unwrap());
    //                     std::process::exit(1);
    //                 }

    //                 args.push(expression.unwrap());
    //             }
    //             self.advance();
    //             Ok(Expression::FunctionCall { calle: expr, args })
    //         } else {
    //             println!("expected ()");
    //             std::process::exit(1);
    //         }
    //     } else {
    //         self.primary()
    //     }
    // }

    fn primary(&mut self) -> Result<Expression, SyntaxError> {
        /*
        primary        → NUMBER | STRING | "true" | "false" | "nil"
              | "(" expression ")" ;
        */
        // println!("{:?}",self.current_token);
        match self.current_token.take() {
            Some(token) => match token.kind {
                TokenType::Nil
                | TokenType::String
                | TokenType::Number
                | TokenType::True
                | TokenType::False
                | TokenType::Identifier => {
                    self.advance();
                    Ok(Expression::Literal { value: token })
                }
                TokenType::LParen => {
                    self.advance();
                    let expr = self.parse_expression();
                    // self.consume(TokenType::RParen);
                    self.advance();

                    return Ok(Expression::GroupingExpression {
                        exp: Box::new(expr.unwrap()),
                    });
                }
                _ => self.generate_syntax_error("Invalid Syntax".to_string()),
            },
            None => self.generate_syntax_error("EOF Error".to_string()),
        }
    }
}
