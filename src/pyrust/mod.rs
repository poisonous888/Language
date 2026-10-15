use crate::pyrust::builder::*;
use crate::pyrust::compiler::*;
use crate::pyrust::parser::*;
use crate::pyrust::vm::*;

mod vm;
mod parser;
mod builder;
mod compiler;

pub fn compile_run(program:String,debug:bool){
    println!("[compiler] lexing...");
    let tokens=tokenize(program,debug);
    println!("[compiler] building...");
    let pre_classes=build(tokens,debug);
    println!("[compiler] compiling...");
    let class =compile(pre_classes,debug);
    println!("[compiler] running...");
    run(class);
}

