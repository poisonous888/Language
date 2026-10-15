use crate::pyrust::builder::*;
use crate::pyrust::compiler::*;
use crate::pyrust::parser::*;
use crate::pyrust::vm::*;

mod vm;
mod parser;
mod builder;
mod compiler;

pub fn compile_run(program:String){
    println!("[compiler] lexing...");
    let tokens=tokenize(program);
    println!("[compiler] building...");
    let pre_classes=build(tokens);
    println!("[compiler] compiling...");
    let class =compile(pre_classes);
    println!("[compiler] running...");
    run(class);
}

