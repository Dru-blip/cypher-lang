
use colored::Colorize;

#[derive(Debug)]
pub struct SyntaxError {
    file_name: String,
    line_no: u32,
    col: u32,
    message: String,
    line:String
}

impl SyntaxError {
    pub fn new(file_name: String, line_no: u32, col: u32, message: String,line:String) -> Self {
        Self {
            file_name,
            line_no,
            col,
            message,
            line
        }
    }
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _=writeln!(f,"{}: {}","Syntax Error".red().bold(),self.message.bold());
        let _=writeln!(f,"{:4} --> {} at line:{} col:{}","",self.file_name.bright_blue(),self.line_no,self.col-1);
        let _=writeln!(f,"{:3} {}","","|");
        write!(f,"{:10}  -> {}","",self.line.underline())
    }
}
