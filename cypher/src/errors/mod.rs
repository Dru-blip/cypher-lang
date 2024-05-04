

pub mod lexical;

pub mod syntax;


#[derive(Debug)]
pub struct RuntimeError{
    message:String
}

impl RuntimeError {
    pub fn new(message:String)->Self{
        Self { 
            message
        }
    }
}