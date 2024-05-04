use std::{collections::HashMap, fs};

use display_json::DisplayAsJsonPretty;
use serde::Serialize;

use crate::objects::Object;

// #[derive(Debug, Clone,Serialize,DisplayAsJsonPretty)]
// pub enum SymbolScope {
//     LOCAL,
//     GLOBAL,
// }

#[derive(Debug, Clone)]
pub struct Symbol{
    pub  depth:u32,
    pub object:Object
}

impl  Symbol {
    fn new(depth:u32,object:Object)->Self {
        Self{
            depth,
            object
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    values: HashMap<String, Symbol>,
    pub next: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new(next: Option<Box<SymbolTable>>) -> Self {
        Self {
            values: HashMap::new(),
            next,
        }
    }
    pub fn get(&self, name: &String) -> Option<&Symbol> {
        let val = self.values.get(name).to_owned();
        if val.is_none() {
            if self.next.as_ref().is_some() {
                return self.next.as_ref().unwrap().get(name);
            }
            return val;
        }
        val
    }

    pub(super) fn depth(name: &String, symbols: &SymbolTable) -> i32 {
        if let Some(_) = symbols.values.get(name) {
            return 1;
        } else {
            return 1 + Self::depth(name, &symbols.next.as_ref().unwrap());
        }
    }

    pub fn define(&mut self, name: String, val: Object,depth:u32) {
        self.values.insert(name, Symbol::new(depth, val));
    }

    pub fn assign(&mut self, name: &String, val: Object,current_depth:u32) ->Object{
        let obj=self.get(name);
        if obj.is_none(){
            return Object::Error(format!("identifer {}",name).to_owned())
        }

        
        Self::assign_util(current_depth, obj.as_ref().unwrap().depth, name, &val, self);
        // let _=fs::write(format!("examples/symbols{}.json",1), self.to_string());
        val
    }

    pub(super) fn assign_util(mut current_depth:u32,target_depth: u32,name: &String,val: &Object, mut symbols: &mut SymbolTable) {
        loop{
            if current_depth==target_depth {
                symbols.values.insert(name.to_owned(),Symbol::new(target_depth,val.to_owned()));
                break;
            }
            
            symbols=symbols.next.as_mut().unwrap();
            current_depth-=1
        }
    }
}
