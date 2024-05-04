
use crate::lexer::token::Location;
use colored::Colorize;


#[derive(Debug)]
pub struct LexicalError{
    pub message: String,
    pub file_name:String,
    pub location:Location,
    pub line:String
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col=self.location.col;
        let _=writeln!(f,"{}: {}","Lexical Error".red().bold(),self.message.bold());
        let _=writeln!(f,"{:4} --> {} at line:{}:{}","",self.file_name.bright_blue(),self.location.line,self.location.col-1);
        let _=writeln!(f,"{:3} {}","","|");
        write!(f,"{:10}  -> {}","",self.line[(col-2) as usize..].underline())
    }
}