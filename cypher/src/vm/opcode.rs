
pub enum Opcode {
    ADD,
    PUSH,
    LC,
    POP,
    SUB,
    MUL,
    DIV,
    MOD,
    PRINT,
    JMP,
    JNE,
    LT,
    GT,
    GOE,
    LOE,
    NOP
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0=>Opcode::ADD,
            1=>Opcode::PUSH,
            2=>Opcode::LC,
            3=>Opcode::POP,
            4=>Opcode::SUB,
            5=>Opcode::MUL,
            6=>Opcode::DIV,
            7=>Opcode::MOD,
            8=>Opcode::PRINT,
            9=>Opcode::JMP,
            10=>Opcode::JNE,
            11=>Opcode::LT,
            12=>Opcode::GT,
            13=>Opcode::GOE,
            14=>Opcode::LOE,
            _=>Opcode::NOP
        }
    }
}