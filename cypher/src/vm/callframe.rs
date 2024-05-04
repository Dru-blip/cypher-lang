use super::{chunk::Chunk, object::Object};



pub struct CallFrame{
    name:String,
    pub code:Vec<u8>,
    pub ip:usize,
    args:Vec<Object>,
    constants:Vec<Object>
}

impl CallFrame {
    pub fn new(name:String,code:Vec<u8>,constants:Vec<Object>,args:Vec<Object>) ->Self{
        Self{
            name,
            code,
            ip:0,
            args,
            constants
        }
    }

 
    pub fn get_code(&self)->&Vec<u8> {
        self.get_code()
    }

    pub fn get_code_length(&self)->usize {
        self.code.len()
    }

    pub fn read_byte(&self)->u8 {
        self.code[self.ip]
    }
    pub fn get_constant(&mut self,index:usize)->&Object{
        self.constants.get(index).unwrap()
    }
    
}