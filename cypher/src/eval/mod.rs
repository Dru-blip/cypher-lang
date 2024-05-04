use std::{collections::HashMap, fs::write};

use crate::{
    lexer::token::TokenType,
    objects::{BuiltinFn, Function, Object},
    parser::expr::{Expression, Program, Statement},
};

use self::symbol_table::SymbolTable;

pub mod symbol_table;

pub struct Eval<'a> {
    program: &'a Program,
    symbols: SymbolTable,
    scope_depth: u32,
    builtins: HashMap<String, Object>,
}

impl<'a> Eval<'a> {
    pub fn new(program: &'a Program) -> Self {
        let mut funcs: HashMap<String, Object> = HashMap::new();
        funcs.insert(
            "len".to_owned(),
            Object::Builtin(BuiltinFn::new(|objects| {
                if objects.len() > 1 {
                    return Object::Error(
                        format!("required 1 args got {}", objects.len()).to_owned(),
                    );
                }
                let obj = &objects[0];
                return match obj {
                    Object::Number(_) => todo!(),
                    Object::Str(s) => Object::Number(s.len() as f64),
                    Object::Bool(_) => todo!(),
                    Object::Nil => todo!(),
                    Object::Error(_) => todo!(),
                    Object::Func(_) => todo!(),
                    Object::Builtin(_) => todo!(),
                    Object::Tuple(_, _) => todo!(),
                    Object::Array(arr) => Object::Number(arr.len() as f64),
                };
                // Object::Nil
            })),
        );

        funcs.insert(
            "push".to_owned(),
            Object::Builtin(BuiltinFn::new(|objects| {
                let obj = &objects[0];
                let args = &objects[1];
                let newobj = match obj {
                    Object::Number(_) => todo!(),
                    Object::Str(s) => Object::Number(s.len() as f64),
                    Object::Bool(_) => todo!(),
                    Object::Nil => todo!(),
                    Object::Error(_) => todo!(),
                    Object::Func(_) => todo!(),
                    Object::Builtin(_) => todo!(),
                    Object::Tuple(_, _) => todo!(),
                    Object::Array(arr) => {
                        let o = match args {
                            Object::Number(_)
                            | Object::Str(_)
                            | Object::Bool(_)
                            | Object::Nil
                            | Object::Func(_)
                            | Object::Tuple(_, _)
                            | Object::Array(_)
                            | Object::Builtin(_) => {
                                let mut new_arr: Vec<Object> = arr.clone();
                                new_arr.push(args.clone());
                                Object::Array(new_arr)
                            }
                            Object::Error(err) => Object::Error(err.to_owned()),
                        };
                        return o;
                    }
                };
                println!("{}", newobj);
                return newobj;
            })),
        );
        Self {
            scope_depth: 1,
            program,
            symbols: SymbolTable::new(None),
            builtins: funcs,
        }
    }
    pub fn run(&mut self) {
        let obj: Object = self.evaluate_statements(&self.program.body);
    }

    fn evaluate_statements(&mut self, statements: &Vec<Statement>) -> Object {
        let mut obj: Option<Object> = None;
        for statement in statements {
            let res = self.evaluate_statement(statement);
            obj = Some(res);
            if obj.is_some() {
                if self.is_error(&obj.as_ref().unwrap()) {
                    println!("{:?}", obj);
                    break;
                }
            }
        }
        if obj.is_none() {
            return Object::Nil;
        }
        obj.unwrap()
    }

    fn is_error(&self, val: &Object) -> bool {
        match val {
            Object::Number(_)
            | Object::Str(_)
            | Object::Bool(_)
            | Object::Func(_)
            | Object::Nil => false,
            Object::Error(_) => true,
            Object::Builtin(_) => false,
            Object::Tuple(_, _) => todo!(),
            Object::Array(_) => false,
        }
    }

    fn end_scope(&mut self) {
        self.symbols = *self.symbols.next.take().unwrap();
    }

    fn begin_scope(&mut self) {
        self.symbols = SymbolTable::new(Some(Box::new(self.symbols.to_owned())));
    }

    fn evaluate_statement(&mut self, statement: &Statement) -> Object {
        match statement {
            Statement::ExpressionStatement { expr } => {
                let val = self.eval_expression(expr);
                val
            }
            Statement::PrintStatement { expr } => {
                let val = self.eval_expression(expr);
                // if val.is_ok() {
                println!("{}", val);
                // }
                Object::Nil
            }
            Statement::VariableStatement { ident, expr } => {
                if expr.is_some() {
                    let initializer = self.evaluate_statement(expr.as_ref().unwrap());
                    if self.is_error(&initializer) {
                        return initializer;
                    }
                    self.symbols.define(
                        ident.value.as_ref().unwrap().to_owned(),
                        initializer.clone(),
                        self.scope_depth,
                    );

                    return initializer;
                }

                self.symbols.define(
                    ident.value.as_ref().unwrap().to_owned(),
                    Object::Nil,
                    self.scope_depth,
                );
                return Object::Nil;
            }
            Statement::IFStatement {
                condition,
                then,
                _else,
            } => self.eval_if_statement(condition, then, _else),
            Statement::BlockStatement { statements } => {
                // self.begin_scope();

                self.symbols = SymbolTable::new(Some(Box::new(self.symbols.to_owned())));
                self.scope_depth += 1;
                // let _= write("examples/block.json", self.symbols.to_string());

                let result = self.evaluate_statements(statements);
                // println!("{:#?}",&self.symbols);

                self.symbols = *self.symbols.next.take().unwrap();
                self.scope_depth -= 1;
                // let _= write("examples/block1.json", self.symbols.to_string());
                // println!("{:#?}",self.symbols);
                // self.end_scope();

                result
            }
            Statement::WhileStatement { condition, body } => {
                let mut condition_expr = self.eval_expression(condition);
                if self.is_error(&condition_expr) {
                    return condition_expr;
                }
                if self.is_truthy(&condition_expr) {
                    let mut obj: Option<Object> = None;
                    // let mut count = 1;
                    while self.is_truthy(&condition_expr) {
                        // if count==5{
                        //     break;
                        // }
                        obj = Some(self.evaluate_statement(&body));
                        condition_expr = self.eval_expression(condition);

                        // println!("condition : {}",&condition_expr);
                        // let _=fs::write(format!("examples/symbols{}.json",count), self.symbols.to_string());
                        // println!("{}",self.symbols);
                        // count+=1;
                    }
                    return obj.unwrap_or(Object::Nil);
                }
                self.end_scope();
                Object::Nil
            }
            Statement::ForStatement {
                initializer,
                condition,
                increment,
                body,
            } => todo!(),
            Statement::FunctionDeclaration { name, args, body } => {
                let func_obj =
                    Object::Func(Function::new(name.clone(), args.to_vec(), body.clone()));
                self.symbols.define(
                    name.to_owned().value.unwrap().to_owned(),
                    func_obj.clone(),
                    self.scope_depth,
                );
                return func_obj;
            }
            Statement::ReturnStatement { expr } => {
                if expr.is_some() {
                    return self.eval_expression(expr.as_ref().unwrap());
                }
                Object::Nil
            }
        }
    }

    fn eval_if_statement(
        &mut self,
        condition: &Expression,
        then: &Statement,
        _else: &Option<Box<Statement>>,
    ) -> Object {
        let condition_expr = self.eval_expression(condition);
        if self.is_error(&condition_expr) {
            return condition_expr;
        }
        if self.is_truthy(&condition_expr) {
            return self.evaluate_statement(then);
        } else {
            return self.evaluate_statement(_else.as_ref().unwrap());
        }
        // Ok(Object::Nil)
    }

    fn is_truthy(&self, value: &Object) -> bool {
        match value {
            Object::Bool(a) => *a,
            Object::Number(_) => true,
            Object::Str(_) => true,
            Object::Nil => false,
            Object::Error(_) => todo!(),
            Object::Func(_) => true,
            Object::Builtin(_) => true,
            Object::Tuple(_, _) => todo!(),
            Object::Array(_) => todo!(),
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Object {
        match expression {
            Expression::VariableAssignment { identifier, expr } => {
                let obj = self.eval_expression(expr);
                self.symbols.assign(
                    identifier.value.as_ref().unwrap(),
                    obj.clone(),
                    self.scope_depth,
                );
                obj
            }
            Expression::GroupingExpression { exp } => {
                return self.eval_expression(exp);
            }
            Expression::UnaryExpression { op, rhs } => todo!(),
            Expression::BinaryExpression { lhs, op, rhs } => {
                let left: Object = self.eval_expression(lhs);
                if self.is_error(&left) {
                    return left;
                }
                let right: Object = self.eval_expression(rhs);

                if self.is_error(&right) {
                    return right;
                }

                let res: Object = match (&left, &right) {
                    (Object::Number(a), Object::Number(b)) => match op.kind {
                        TokenType::Plus => Object::Number(a + b),
                        TokenType::Minus => Object::Number(a - b),
                        TokenType::Star => Object::Number(a * b),
                        TokenType::Slash => Object::Number(a / b),
                        TokenType::Modulo => Object::Number(a % b),
                        TokenType::GreaterThan => Object::Bool(a > b),
                        TokenType::GreaterThanOrEqual => Object::Bool(a >= b),
                        TokenType::LessThan => Object::Bool(a < b),
                        TokenType::LessThanOrEqual => Object::Bool(a <= b),
                        TokenType::Equal => Object::Bool(a == b),
                        TokenType::NotEqual => Object::Bool(a != b),
                        _ => Object::Error(format!(
                            "Unknown operation between {} {:?} {}",
                            &left, op.kind, &right
                        )),
                    },
                    (Object::Bool(a), Object::Bool(b)) => match op.kind {
                        TokenType::And => return Object::Bool(*a && *b),
                        TokenType::Or => return Object::Bool(*a || *b),
                        _ => Object::Error(format!(
                            "Unknown operation between {} {:?} {}",
                            &left, op.kind, &right
                        )),
                    },
                    _ => Object::Error(format!(
                        "Unknown operation  between {} {:?} {}",
                        &left, op.kind, &right
                    )),
                };
                res
            }
            Expression::IncrementDecrement { op, identifier } => {
                let obj = self.symbols.get(identifier.value.as_ref().unwrap());

                // println!("{:?}",obj);
                if obj.is_none() {
                    return Object::Error(
                        format!(
                            "identifier {} not found",
                            identifier.value.as_ref().unwrap()
                        )
                        .to_owned(),
                    );
                }

                let new_obj: Object = match op.kind {
                    TokenType::Increment => match &obj.unwrap().object {
                        Object::Number(num) => Object::Number(num + 1.0),
                        _ => Object::Error(format!(
                            "Unknown operation between {} {:?}",
                            &identifier, op.kind,
                        )),
                    },
                    TokenType::Decrement => match &obj.unwrap().object {
                        Object::Number(num) => Object::Number(num - 1.0),
                        _ => Object::Error(format!(
                            "Unknown operation between {} {:?}",
                            &identifier, op.kind,
                        )),
                    },
                    _ => Object::Error(format!("Unknown operation  {}", op)),
                };

                // println!("{}",new_obj);
                self.symbols.assign(
                    identifier.value.as_ref().unwrap(),
                    new_obj.clone(),
                    self.scope_depth,
                );
                new_obj
            }

            Expression::FunctionCall { calle, args } => {
                let obj = self.eval_expression(calle);

                // Object::Nil
                // let func = self.symbols.get(calle.value.as_ref().unwrap());
                // if func.is_none() {
                //     let builtin = self.builtins.get(calle.value.as_ref().unwrap());
                //     if builtin.is_none() {
                //         return Object::Error(
                //             format!("Function {}() not found", calle.value.as_ref().unwrap())
                //                 .to_string(),
                //         );
                //     }

                //     match builtin.as_ref().unwrap() {
                //         Object::Builtin(builtin) => {
                //             let f=builtin.Fn;
                //             let mut eval_args: Vec<Object> = vec![];

                //             for expression in args {
                //                 let obj = self.eval_expression(expression);
                //                 eval_args.push(obj);
                //             }
                //             return f(eval_args);
                //             // println!("{:?}",);
                //         },
                //         _=>todo!()
                //     }

                //     return  Object::Nil;
                // }

                let obj = self.eval_function_call(obj, args);
                obj
            }
            Expression::Literal { value } => match value.kind {
                TokenType::Identifier => {
                    let obj = self.symbols.get(value.value.as_ref().unwrap());
                    // println!("{:?}",obj);
                    if obj.is_none() {
                        // println!("{:?}",obj);
                        let builtin = self.builtins.get(value.value.as_ref().unwrap());
                        if builtin.is_some() {
                            return builtin.unwrap().clone();
                        }

                        return Object::Error(
                            format!("Identifier {} not found", value.value.as_ref().unwrap())
                                .to_string(),
                        );
                    }
                    obj.unwrap().object.clone()
                }
                TokenType::Number => {
                    let number: f64 = value.value.as_ref().unwrap().parse().unwrap_or(0.0);
                    return Object::Number(number);
                }
                TokenType::String => return Object::Str(value.value.as_ref().unwrap().to_owned()),
                TokenType::True => return Object::Bool(true),
                TokenType::False => return Object::Bool(false),
                _ => return Object::Nil,
            },
            Expression::GetExpression { identifier, exp } => {
                let obj = self.eval_expression(exp);

                if self
                    .builtins
                    .contains_key(identifier.value.as_ref().unwrap())
                {
                    let f = self
                        .builtins
                        .get(identifier.value.as_ref().unwrap())
                        .unwrap()
                        .to_owned();
                    return Object::Tuple(Box::new(obj), Box::new(f));
                }
                Object::Nil
            }
            Expression::ArrayDeclaration { elements } => {
                let mut objects: Vec<Object> = vec![];
                for element in elements {
                    let obj = self.eval_expression(element);
                    if self.is_error(&obj) {
                        return obj;
                    }
                    objects.push(obj)
                }
                Object::Array(objects)
            }
            Expression::ArrayIndexing { ident, index } => {
                let obj = self.eval_expression(ident);
                if self.is_error(&obj) {
                    return obj;
                }
                let arr_index = self.eval_expression(index);

                if self.is_error(&arr_index) {
                    return arr_index;
                }

                match obj {
                    Object::Array(arr) => match arr_index {
                        Object::Number(index) => {
                            if let Some(obj) = arr.get(index as usize) {
                                return obj.clone();
                            }
                            return Object::Error("Unable to index ,{arr#?}".to_owned());
                        }
                        _ => Object::Error(format!("expected Number as index ").to_owned()),
                    },
                    _ => Object::Error(format!("{}", obj)),
                }
            }
        }
    }

    fn eval_function_call(&mut self, obj: Object, func_args: &Vec<Expression>) -> Object {
        let mut eval_args: Vec<Object> = vec![];

        for expression in func_args {
            let obj = self.eval_expression(expression);
            eval_args.push(obj);
        }

        match obj {
            Object::Func(fun_obj) => {
                if func_args.len() < fun_obj.args.len() {
                    return Object::Error(
                        format!(
                            "Expected {} arguments but got {}",
                            fun_obj.args.len(),
                            func_args.len()
                        )
                        .to_owned(),
                    );
                }

                self.begin_scope();
                self.scope_depth += 1;

                for index in 0..fun_obj.args.len() {
                    let name = &fun_obj.args[index];
                    let val = &eval_args[index];
                    self.symbols.define(
                        name.value.as_ref().unwrap().to_owned(),
                        val.to_owned(),
                        self.scope_depth,
                    );
                }

                let obj = self.evaluate_statement(&fun_obj.body);

                self.end_scope();
                self.scope_depth -= 1;

                obj
            }
            Object::Builtin(func) => {
                let function = func.Fn;
                function(eval_args)
            }
            Object::Tuple(caller, func) => {
                // (builtfn.Fn)();

                match *func {
                    Object::Builtin(func) => {
                        return (func.Fn)(vec![*caller, Object::Array(eval_args)])
                    }
                    _ => todo!(),
                }
            }
            _ => Object::Error(format!("is not callable")),
        }
    }
}
