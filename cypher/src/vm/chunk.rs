use super::object::Object;



pub struct Chunk{
    pub name:String,
    pub code:Vec<u8>,
    pub constants:Vec<Object>,
    pub depth:usize
}

impl Chunk {
    pub fn new(name:String)->Self {
        Self{
            name,
            code:vec![],
            constants:vec![],
            depth:0,
        }
    }

    pub fn write_byte(&mut self,byte:u8) {
        self.code.push(byte)
    }

    pub fn add_constant(&mut self,obj:Object) ->usize{
        self.constants.push(obj);
        self.constants.len()-1
    }

    pub fn get_code(&self)->&Vec<u8> {
        &self.code
    }
    
    pub fn get_code_length(&self)->usize {
        self.code.len()
    }
    
}