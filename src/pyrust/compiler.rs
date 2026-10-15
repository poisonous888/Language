use crate::pyrust::builder::*;
use crate::pyrust::compiler::Instruction::*;
use crate::pyrust::parser::*;
use crate::pyrust::parser::Token::*;
pub fn compile(pre:ClassPreProcess)->Class{
    loop{
    
    };
}
fn get_cur_inst(statement:Vec<Token>)->Vec<Instruction>{
    let mut instruction_set:Vec<Instruction>=Vec::new();
    match statement.first().expect("[ERROR] First token in statement is empty") {
        Identifier(e)=>{
            let next = statement.get(1).expect("Second token in bind statement is empty");
            match next {
                Assign=>{
                    instruction_set.push(BindAssign(e.clone()));
                    println!("Bind assign - {}",e)
                }
                _=>{println!("Dangling identifier - {}",e)}
            }
        }
        Bind=>{
            let identify= statement.get(1).expect("[ERROR] Second token in bind statement is empty");
            match identify {
                Identifier(e)=>{
                    if ["it","io","here"].contains(&&**e){panic!("[ERROR] Cannot bind to identifier - {}",e)}
                    instruction_set.push(BindNew(e.clone()));
                    println!("Bind new - {}",e)
                }
                _=>{panic!("[ERROR] Cannot bind to a non-identifier")}
            }
        }
        Scope=>{
            let identify= statement.get(1).expect("[ERROR] Second token in bind statement is empty");
            match identify {
                Identifier(e)=>{
                    if ["it","io","here"].contains(&&**e){panic!("[ERROR] Cannot bind to identifier - {}",e)}
                    instruction_set.push(BindNew(e.clone()));
                    println!("Bind new - {}",e)
                }
                _=>{panic!("[ERROR] Cannot bind to a non-identifier")}
            }
        }
        InsEnd=>{}
        _=>{println!("Unknown")}
    }
    instruction_set
}

pub struct Class{
    pub instructions:Vec<Instruction>,
    pub sub_classes:Vec<Class>,
    pub labels:Vec<(String,i32)>
}
impl Class{
    fn new()->Self{
        Class{
            instructions:Vec::new(),
            sub_classes:Vec::new(),
            labels:Vec::new()
        }
    }
}
#[derive(Debug)]
pub enum Instruction{
    BindNew(String),
    BindAssign(String),
    Expression(Vec<ExpressionObj>),
}
#[derive(Debug)]
pub enum ExpressionObj{

}