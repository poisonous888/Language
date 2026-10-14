use crate::pyrust::parser::parse;

mod vm;
mod parser;

pub fn compile_run(program:String){
    println!("[compiler] compiling and running...");
    parse(program);
    
}

