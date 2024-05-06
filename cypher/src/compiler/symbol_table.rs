use std::collections::HashMap;

use crate::lexer::token::Token;



pub enum SymbolScope {
    LOCAL,
    GLOBAL
}

pub struct Symbol{
    name:String,
    depth:usize,
    scope:SymbolScope,
    pub index:usize
}

pub struct SymbolTable{
    values:HashMap<String,Symbol>,
    next:Option<Box<SymbolTable>>
}

impl SymbolTable {
    pub fn new()->Self {
        Self{
            values:HashMap::new(),
            next:None
        }
    }

    pub fn define(&mut self,name:String,depth:usize,scope:SymbolScope,index:usize) {
        let symbol=Symbol{name:name.to_owned(),depth,scope,index};
       
        self.values.insert(name, symbol);
    }

    pub fn resolve(&self,name:&String)->Option<&Symbol> {
        self.values.get(name)
    }
}