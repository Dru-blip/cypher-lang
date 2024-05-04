use super::{chunk::Chunk, opcode::Opcode};



pub struct Disassembler<'a>{
    chunk:&'a Chunk,
    ip:usize
}


impl<'a> Disassembler<'a>  {
    pub fn new(chunk:&'a Chunk) ->Self{
        Self{
            chunk,
            ip:0
        }
    }

    pub fn run(&mut self) {
        let len=self.chunk.get_code_length();
        println!("{}:",&self.chunk.name);
        while  self.ip<len{
            let ins=self.decode();
            self.disassemble_ins(ins)
        }
    }

    fn decode(&mut self)->Opcode{
        let opcode=self.chunk.code[self.ip];
        self.ip+=1;
        Opcode::from(opcode)
    }

    fn disassemble_ins(&mut self,ins:Opcode) {
        match ins {
            Opcode::ADD => {
                println!("{:5} add"," ");
            },
            Opcode::PUSH => {
                println!("push")
            },
            Opcode::LC => {
                self.disassemble_load_ins();
            },
            Opcode::POP => {
                println!("pop")
            },
            Opcode::SUB => {
                println!("sub")
            },
            Opcode::MUL => {
                println!("{:5} mul"," ");
            },
            Opcode::DIV => {
                println!("div")
            },
            Opcode::MOD => {
                println!("mod")
            },
            Opcode::PRINT => {
                println!("{:5} print"," ")
            },

            Opcode::JMP => {
                let index=self.chunk.code[self.ip];
                // self.ip+=1;
                let ip=&self.chunk.code[self.ip];
                self.ip+=1;
                print!("{:5} jmp"," ");
                println!("{:3} {}"," ",ip);
            },
            Opcode::JNE => {
                self.disassemble_jne_ins()
            },
            Opcode::LT => {
                println!("{:5} le"," ");
            },
            Opcode::GT => {
                println!("{:5} ge"," ");
            },
            Opcode::GOE => todo!(),
            Opcode::LOE => todo!(),
            Opcode::NOP => {
                println!("nop")
            },
        }
    }

    fn disassemble_load_ins(&mut self) {
        let index=self.chunk.code[self.ip];
        let constant=&self.chunk.constants[index as usize];
        self.ip+=1;
        print!("{:5} lc"," ");
        println!("{:4} {}"," ",constant);
    }

    fn disassemble_jne_ins(&mut self) {
        let index=self.chunk.code[self.ip];
        let ip=&self.chunk.code[self.ip];
        self.ip+=1;
        print!("{:5} jne"," ");
        println!("{:3} {}"," ",ip);
    }

    
}