use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::errors::lexical::LexicalError;

use super::token::{Location, Token, TokenList, TokenType};

pub struct Scanner<'a> {
    raw: &'a str,
    lines: &'a Vec<&'a str>,
    current_char: Option<char>,
    chars: Peekable<Chars<'a>>,
    reserved_words: HashMap<String, TokenType>,
    global_location: Location,
    file_name: String,
}

impl<'a> Scanner<'a> {
    pub fn new(raw: &'a str, file_name: String, lines: &'a Vec<&'a str>) -> Self {
        let mut reserved: HashMap<String, TokenType> = HashMap::new();

        reserved.insert("and".to_string(), TokenType::And);
        reserved.insert("or".to_string(), TokenType::Or);
        reserved.insert("while".to_string(), TokenType::While);
        reserved.insert("for".to_string(), TokenType::For);
        reserved.insert("do".to_string(), TokenType::Do);
        reserved.insert("break".to_string(), TokenType::Break);
        reserved.insert("continue".to_string(), TokenType::Continue);
        reserved.insert("until".to_string(), TokenType::Until);
        reserved.insert("def".to_string(), TokenType::Function);
        reserved.insert("if".to_string(), TokenType::If);
        reserved.insert("elseif".to_string(), TokenType::Elseif);
        reserved.insert("else".to_string(), TokenType::Else);
        reserved.insert("repeat".to_string(), TokenType::Repeat);
        reserved.insert("in".to_string(), TokenType::In);
        reserved.insert("print".to_string(), TokenType::Print);
        reserved.insert("return".to_string(), TokenType::Return);
        reserved.insert("nil".to_string(), TokenType::Nil);
        reserved.insert("goto".to_string(), TokenType::Goto);
        reserved.insert("true".to_string(), TokenType::True);
        reserved.insert("false".to_string(), TokenType::False);
        reserved.insert("not".to_string(), TokenType::Not);
        reserved.insert("let".to_string(), TokenType::Let);

        Self {
            raw,
            lines,
            current_char: None,
            chars: raw.chars().peekable(),
            reserved_words: reserved,
            global_location: Location::new(1, 1, 0),
            file_name,
        }
    }

    pub fn get_location(&self) -> Location {
        self.global_location
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) {
        self.current_char = self.chars.next();
        self.global_location.col += 1;
        self.global_location.index += 1;
    }

    fn create_token(
        &self,
        token_type: TokenType,
        value: Option<String>,
    ) -> Result<Token, LexicalError> {
        let location = self.global_location;
        Ok(Token::new(token_type, location, value))
    }

    fn generate_lex_error(&self, message: String) -> Result<Token, LexicalError> {
        let line = self.lines[(self.global_location.line - 1) as usize];
        let error = LexicalError {
            message,
            location: self.global_location,
            file_name: self.file_name.to_owned(),
            line: line.to_string(),
        };
        Err(error)
    }

    fn check_op_return(
        &mut self,
        op: char,
        true_type: TokenType,
        false_type: TokenType,
    ) -> TokenType {
        if let Some(ch) = self.peek() {
            if *ch == op {
                self.advance();
                return true_type;
            }
        }
        false_type
    }

    fn scan_operator(&mut self, op: char) -> Result<Token, LexicalError> {
        let operator: TokenType = match op {
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Modulo,
            ';' => TokenType::SemiColon,
            ',' => TokenType::Comma,
            '}' => TokenType::RBrace,
            '{' => TokenType::LBrace,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ']' => TokenType::RSqBracket,
            '[' => TokenType::LSqBracket,
            '.' => TokenType::Dot,
            '+' => self.check_op_return('+', TokenType::Increment, TokenType::Plus),
            '-' => self.check_op_return('-', TokenType::Decrement, TokenType::Minus),
            '>' => self.check_op_return('=', TokenType::GreaterThanOrEqual, TokenType::GreaterThan),
            '<' => self.check_op_return('=', TokenType::LessThanOrEqual, TokenType::LessThan),
            '!' => self.check_op_return('=', TokenType::NotEqual, TokenType::Bang),
            '=' => self.check_op_return('=', TokenType::Equal, TokenType::Assign),
            _ => return self.generate_lex_error("cannot recognize operator".to_string()),
        };
        self.create_token(operator, None)
    }

    fn scan_number(&mut self, ch: char) -> Result<Token, LexicalError> {
        let mut num = String::from(ch);
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                num.push(*c);
                self.advance()
            } else {
                break;
            }
        }

        self.create_token(TokenType::Number, Some(num))
    }

    fn scan_string(&mut self) -> Result<Token, LexicalError> {
        let mut string = String::from("");
        loop {
            let ch = self.peek();
            if ch.is_none()  {
                return Err(LexicalError {
                    file_name: self.file_name.to_owned(),
                    message: "unterminated string literal".to_owned(),
                    location: self.global_location,
                    line: self.lines[(self.global_location.line-1) as usize].to_owned(),
                });
            }
            if ch.unwrap().is_alphanumeric() && *ch.unwrap() != '"' {
                string.push(*ch.unwrap());
                self.advance();
                continue;
            }
            break;
        }
        self.advance();
        self.create_token(TokenType::String, Some(string))
    }

    fn scan_ident_or_keyword(&mut self, ch: char) -> Result<Token, LexicalError> {
        let mut string = String::from(ch);
        while let Some(c) = self.peek() {
            if c.is_alphabetic() || *c == '_' {
                string.push(*c);
                self.advance();
                continue;
            }
            break;
        }

        match string.as_str() {
            "and" | "print" | "false" | "let" | "true" | "do" | "while" | "for" | "def" | "nil"
            | "or" | "not" | "until" | "if" | "else" | "elseif" | "goto" | "return" | "repeat"
            | "in" => self.create_token(
                *self.reserved_words.get(string.as_str()).unwrap(),
                Some(string),
            ),
            _ => self.scan_identifier(string),
        }
    }

    fn scan_identifier(&self, value: String) -> Result<Token, LexicalError> {
        self.create_token(TokenType::Identifier, Some(value))
    }

    fn skip_whitespaces(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.global_location.line += 1;
                    self.global_location.col = 0;
                    self.advance();
                    continue;
                }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        self.skip_whitespaces();
        self.advance();
        let token: Result<Token, LexicalError> = match self.current_char {
            Some(ch) => match ch {
                '+' | '-' | '*' | '/' | '%' | '=' | '>' | '<' | '{' | '}' | '(' | ')' | '['
                | ']' | ';' | '.' | ',' | '!' => self.scan_operator(ch),
                '0'..='9' => self.scan_number(ch),
                '"' => self.scan_string(),
                _ => {
                    if ch.is_alphabetic() || ch == '_' {
                        self.scan_ident_or_keyword(ch)
                    } else {
                        self.generate_lex_error("unable to identify character".to_string())
                    }
                }
            },
            None => self.create_token(TokenType::Eof, None),
        };
        token
    }

    pub fn generate_token_list(lexer: &mut Scanner) -> TokenList {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lexer.next_token();
            if token.is_ok() {
                if token.as_ref().unwrap().kind == TokenType::Eof {
                    break;
                }
                tokens.push(token.unwrap())
            }
        }
        return TokenList::new(tokens);
    }
}
