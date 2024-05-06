use std::process::ExitCode;

use crate::{
    lexer::token::{Token, TokenType},
    parser::expr::{Expression, Program, Statement},
    vm::{chunk::Chunk, object::Object, opcode::Opcode},
};

use super::symbol_table::{Symbol, SymbolScope, SymbolTable};

pub struct Compiler {
    chunk: Chunk,
    filename: String,
    scope_depth: usize,
    symboltable:SymbolTable
}

impl Compiler {
    pub fn new(filename: String) -> Self {
        Self {
            chunk: Chunk::new(filename.to_owned()),
            filename,
            scope_depth: 0,
            symboltable:SymbolTable::new()
        }
    }

    fn emit_opcode(&mut self, operator: &Token) {
        // println!("{}",operator);
        match operator.kind {
            TokenType::Plus => self.chunk.write_byte(Opcode::ADD as u8),
            TokenType::Minus => self.chunk.write_byte(Opcode::SUB as u8),
            TokenType::Star => self.chunk.write_byte(Opcode::MUL as u8),
            TokenType::Slash => self.chunk.write_byte(Opcode::DIV as u8),
            TokenType::Modulo => self.chunk.write_byte(Opcode::MOD as u8),
            TokenType::GreaterThan => self.chunk.write_byte(Opcode::GT as u8),
            TokenType::GreaterThanOrEqual => self.chunk.write_byte(Opcode::GOE as u8),
            TokenType::LessThan => self.chunk.write_byte(Opcode::LT as u8),
            TokenType::LessThanOrEqual => self.chunk.write_byte(Opcode::LOE as u8),
            _ => self.chunk.write_byte(Opcode::NOP as u8),
        }
    }

    pub fn compile_program(mut self, program: Program) -> Chunk {
        self.compile_statements(&program.body);
        self.chunk
    }

    fn compile_statements(&mut self, statements: &Vec<Statement>) {
        for statement in statements {
            self.compile_statement(statement)
        }

        
    }

    fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ExpressionStatement { expr } => self.compile_expression(expr),
            Statement::PrintStatement { expr } => {
                self.compile_expression(expr);
                self.chunk.write_byte(Opcode::PRINT as u8)
            }
            Statement::VariableStatement { ident, expr } => {
                if let Some(statement)=expr {
                    self.compile_statement(statement);
                }
                else{
                    self.chunk.add_constant(Object::Nil);
                }
                self.symboltable.define(ident.value.as_ref().unwrap().to_owned(),self.scope_depth,SymbolScope::GLOBAL,self.chunk.constants.len()-1);
                
            },
            Statement::IFStatement {
                condition,
                then,
                _else,
            } => {
                self.compile_expression(condition);
                self.chunk.write_byte(Opcode::JNE as u8);
                self.chunk.write_byte(199);
                let jmp_index = self.chunk.get_code_length() - 1;
                
                self.compile_statement(then);
                self.chunk.write_byte(Opcode::JMP as u8);
                self.chunk.write_byte(199);
                let then_index = self.chunk.get_code_length() - 1;
                // self.chunk.code[then_index-1]=(self.chunk.get_code_length()-1) as u8;

                if _else.is_some() {
                    self.compile_statement(_else.as_ref().unwrap());
                    let else_index=self.chunk.get_code_length();

                    self.chunk.code[jmp_index] = (then_index+1) as u8;
                    self.chunk.code[then_index]=else_index as u8;
                } else {
                    self.chunk.code[jmp_index] = then_index as u8;
                }
            }
            Statement::BlockStatement { statements } => {
                self.compile_statements(statements);
            }
            Statement::WhileStatement { condition, body } => {
                self.compile_expression(condition);
                self.chunk.write_byte(Opcode::JNE as u8);
                self.chunk.write_byte(144);
                let cond_index = self.chunk.get_code_length() - 1;
                self.compile_statement(body);

                self.chunk.code[cond_index] = (self.chunk.get_code_length() - 1) as u8;
            }
            Statement::ForStatement {
                initializer,
                condition,
                increment,
                body,
            } => todo!(),
            Statement::FunctionDeclaration { name, args, body } => todo!(),
            Statement::ReturnStatement { expr } => todo!(),
        }
    }

    fn compile_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::VariableAssignment { identifier, expr } => {
                self.compile_expression(expr);
                if let Some(symbol)=self.symboltable.resolve(identifier.value.as_ref().unwrap()){
                    
                }
            },
            Expression::GroupingExpression { exp } => {
                self.compile_expression(exp);
            }
            Expression::UnaryExpression { op, rhs } => {
                self.compile_expression(rhs);
                self.emit_opcode(op)
            }
            Expression::BinaryExpression { lhs, op, rhs } => {
                self.compile_expression(lhs);
                self.compile_expression(rhs);
                self.emit_opcode(op)
            }
            Expression::IncrementDecrement { op, identifier } => {

                self.emit_opcode(op);
            },
            Expression::ArrayDeclaration { elements } => todo!(),
            Expression::ArrayIndexing { ident, index } => todo!(),
            Expression::FunctionCall { calle, args } => todo!(),
            Expression::GetExpression { identifier, exp } => todo!(),
            Expression::Literal { value } => match value.kind {
                TokenType::Number => {
                    self.chunk.write_byte(Opcode::LC as u8);
                    let index = self.chunk.add_constant(Object::Number(
                        value
                            .value
                            .as_ref()
                            .unwrap()
                            .to_owned()
                            .parse()
                            .unwrap_or(0.0),
                    ));
                    self.chunk.write_byte(index as u8);
                }
                TokenType::Identifier=>{
                    if let Some(symbol)=self.symboltable.resolve(value.value.as_ref().unwrap()){
                        self.chunk.write_byte(Opcode::GETGLOBAL as u8);
                        self.chunk.write_byte(symbol.index as u8);
                    }
                    // self.chunk.write_byte(Opcode::GETGLOBAL as u8);
                }
                TokenType::True=>{
                    self.chunk.write_byte(Opcode::LC as u8);
                    let index=self.chunk.add_constant(Object::Boolean(true));
                    self.chunk.write_byte(index as u8);
                }
                TokenType::False=>{
                    self.chunk.write_byte(Opcode::LC as u8);
                    let index=self.chunk.add_constant(Object::Boolean(false));
                    self.chunk.write_byte(index as u8);
                }
                TokenType::String=>{
                    self.chunk.write_byte(Opcode::LC as u8);
                    let index=self.chunk.add_constant(Object::Str(value.value.as_ref().unwrap().to_owned()));
                    self.chunk.write_byte(index as u8);
                }
                _ => todo!(),
            },
        }
    }
}
