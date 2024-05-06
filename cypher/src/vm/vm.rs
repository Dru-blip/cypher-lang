use super::{callframe::CallFrame, chunk::Chunk, object::{self, Object}, opcode::Opcode};

pub struct VM {
    stack: Vec<Object>,
    frames:Vec<CallFrame>,
    fp:usize,
    sp:usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            sp:0,
            fp:0,
            frames:Vec::with_capacity(1024),
            stack: Vec::with_capacity(256),
        }
    }

    pub fn run(&mut self,code: Chunk) {
        let frame=CallFrame::new("main".to_owned(),code.code,code.constants,vec![]);
        self.add_frame(frame);
        while self.get_current_frame().ip < self.get_current_frame().get_code_length() {
            let opcode = self.decode_opcode();
            self.execute_instruction(opcode)
        }

        // println!("{:?}",self.stack)
    }

    fn add_frame(&mut self,frame:CallFrame) {
        self.frames.push(frame);
        self.fp+=1;
    }

    fn is_truthy(&self,object:&Object)->bool{
        match object {
            Object::Number(_) => true,
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Str(_) => true,
        }
    }

    fn execute_instruction(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::ADD => {
                let left=self.pop();
                let right=self.pop();
                match (left,right) {
                    (Object::Number(a), Object::Number(b)) => self.push(Object::Number(a+b)),
                    _=>todo!()
                }
            }
            Opcode::PUSH => {
                
            }
            Opcode::LC=>{
                let index=self.read_next_8bytes();
                let obj=self.get_current_frame().get_constant(index as usize).to_owned();
                self.push(obj);
            },
            Opcode::POP => todo!(),
            Opcode::SUB => todo!(),
            Opcode::MUL => todo!(),
            Opcode::DIV => todo!(),
            Opcode::MOD => todo!(),
            Opcode::PRINT => {
                println!("{}",self.pop());
            },
            Opcode::NOP => todo!(),
            Opcode::JMP => {
                let index=self.read_next_8bytes();
                self.get_current_frame().ip=index as usize;
            },
            Opcode::JNE => {
                let condition=self.pop();
                if !self.is_truthy(&condition){
                    self.get_current_frame().ip+=1;
                }
                else{
                    let index=self.read_next_8bytes();
                    self.get_current_frame().ip=index as usize;
                }
              
            },
            Opcode::LT => {
                let left=self.pop();
                let right=self.pop();
                match (left,right) {
                    (Object::Number(a), Object::Number(b)) => self.push(Object::Boolean(a<b)),
                    _=>todo!()
                }
            },
            Opcode::GT => todo!(),
            Opcode::GOE => todo!(),
            Opcode::LOE => todo!(),
            Opcode::GETGLOBAL => {
                let index=self.read_next_8bytes();
                let obj=self.get_current_frame().get_constant(index as usize).to_owned();
                self.push(obj);
            },
            Opcode::SETGLOBAL => todo!(),
            Opcode::REASSIGN => todo!(),
            Opcode::GETLOCAL => todo!(),
            Opcode::SETLOCAL => todo!(),
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let byte = self.get_current_frame().read_byte();
        self.get_current_frame().ip += 1;
        Opcode::from(byte)
    }
    

    fn read_next_8bytes(&mut self) -> u8 {
        let byte = self.get_current_frame().read_byte();
        self.get_current_frame().ip += 1;
        byte
    }

    fn push(&mut self,val:Object) {
        self.stack.push(val);
        self.sp+=1;
    }

    fn pop(&mut self)->Object {
        self.sp-=1;
        self.stack.pop().unwrap_or(Object::Nil)
    }

    fn get_current_frame(&mut self)->&mut CallFrame{
        &mut self.frames[self.fp-1]
    }

}
