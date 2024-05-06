use std::fs;

use cypher::compiler::compiler::Compiler;
use cypher::lexer::scanner::Scanner;
use cypher::parser::parser::Parser;
use cypher::vm::{chunk::Chunk, disassemble::Disassembler, object::Object, vm::VM};



fn main() {
    let filename="examples/hello.cy".to_owned();
    let code = fs::read_to_string(filename.to_owned()).unwrap();

    /* 
       vector containing raw string literals by line num
    */
    let lines:Vec<&str>=code.split("\n").collect();
    let mut lex = Scanner::new(&code,filename.to_owned(),&lines);
    let mut par=Parser::new(&mut lex,&filename,&lines);
    let program = par.parse_program();
    // let  mut evaluator=Eval::new(&program);
    let _=fs::write("examples/hello.json", program.to_string().as_str());
    // println!("{}",program.to_string().as_str());
    // evaluator.run();
    let compiler=Compiler::new(filename.to_owned());

    let chunk=compiler.compile_program(program);

    // let mut chunk:Chunk =Chunk::new("Main".to_owned());

    // chunk.write_byte(2);
    // let index=chunk.add_constant(Object::Number(3.0));
    // chunk.write_byte(index as u8);

    // chunk.write_byte(2);
    // let index=chunk.add_constant(Object::Number(4.0));
    // chunk.write_byte(index as u8);

    // chunk.write_byte(5);
    let mut dis:Disassembler=Disassembler::new(&chunk);
    dis.run();

    let mut vm=VM::new();

    vm.run(chunk);


}
