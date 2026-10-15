use crate::pyrust::builder::*;
use crate::pyrust::compiler::Instruction::*;
use crate::pyrust::compiler::Literal::{ClassDef, DoubleLiteral, IntLiteral, StringLiteral};
use crate::pyrust::parser::*;
use crate::pyrust::parser::Opperator::Add;
use crate::pyrust::parser::Token::*;
pub fn compile(pre:ClassPreProcess,debug:bool)->Class{
    let mut new =Class::new();
    for thing in pre.sub_classes{
        new.sub_classes.push(compile(thing,debug))
    };
    for thing in pre.instructions{
        let mut ins =get_inst(thing,debug);
        new.instructions.append(&mut ins);
    }
    if debug{println!("{:?}",new)}
    new
}
fn get_inst(statement: Vec<Token>,debug:bool) ->Vec<Instruction>{
    let mut instruction_set:Vec<Instruction>=Vec::new();
    match index_of(&statement,Bind){
        None=>{}
        Some(i)=>{
            if debug{println!("Bind")}
            if i>0{ panic!("[ERROR] bind must be the first token in a statement") }
            let next=statement.get(i as usize+1).expect("VERY BAD BIND ERROR");
            match next {
                Identifier(e)=>{
                    if debug{println!(" - name: {}",e)}
                    instruction_set.push(BindNew(e.clone()));
                    let third=statement.get(i as usize+2);
                    match third {
                        Some(Assign)=>{
                            if debug{println!(" - assigned")}
                            let mut sub =statement.clone();
                            sub.remove(0);
                            sub.remove(0);
                            sub.remove(0);
                            let mut expr =get_inst(sub,debug);
                            instruction_set.append(&mut expr);
                            instruction_set.push(BindAssign(e.clone()));
                        }
                        _=>{if debug{println!(" - unassigned")}}
                    }
                }
                _=>{panic!("[ERROR] cannot bind to non-identifier")}
            }
            return instruction_set
        }
    }
    match index_of(&statement,Assign){
        None=>{}
        Some(i)=>{
            if debug{println!("Assign")}
            if i!=1{ panic!("[ERROR] assign must be the 2nd token in a statement") }
            let prev=statement.get(i as usize-1).expect("VERY BAD ASSIGN ERROR");
            match prev{
                Identifier(e)=>{
                    if debug{println!(" - assigned")}
                    let mut sub =statement.clone();
                    sub.remove(0);
                    sub.remove(0);
                    let mut expr =get_inst(sub,debug);
                    instruction_set.append(&mut expr);
                    instruction_set.push(BindAssign(e.clone()));
                }
                _=>{panic!("[ERROR] cannot assign to non-identifier")}
            }
            return instruction_set
        }
    }
    match index_of(&statement,Oppr(Add)){
        None=>{}
        Some(i)=>{
            if debug{println!("Expression")}
            if i!=1{panic!("[ERROR] expression must be between 2 other tokens")}
            let prev=statement.get(0).expect("VERY BAD EXPRESSION ERROR");
            match prev {
                Identifier(e)=>{
                    instruction_set.push(PushBind(e.clone()));
                }
                NumLit(e)=>{
                    let val=match e{
                        Number::Int(e) => {IntLiteral(*e)}
                        Number::Double(e) => {DoubleLiteral(*e)}
                    };
                    instruction_set.push(PushLiteral(val));
                }
                _=>{panic!("[ERROR] cannot express on that token")}
            }
        }
    }
    match statement.first() {
        Some(e)=>{
            match e{
                NumLit(e)=>{
                    let val=match e{
                        Number::Int(e) => {IntLiteral(*e)}
                        Number::Double(e) => {DoubleLiteral(*e)}
                    };
                    instruction_set.push(PushLiteral(val));
                }
                StringLit(e)=>{
                    instruction_set.push(PushLiteral(StringLiteral(e.clone())));
                }
                Scope(i)=>{
                    instruction_set.push(PushLiteral(ClassDef(*i)));
                }
                e=>{
                    if debug{println!("Unknown")}
                    if debug{println!(" - {:?}",e)}
                }
            }
        }
        None=>{
        
        }
    }
    instruction_set
}
#[derive(Debug)]
pub enum Instruction{
    BindNew(String),
    BindAssign(String),
    PushBind(String),
    PushLiteral(Literal),
}
#[derive(Debug)]
pub enum Literal{
    StringLiteral(String),
    IntLiteral(i32),
    DoubleLiteral(f64),
    ClassDef(i32),
}
#[derive(Debug)]
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
pub enum ExpressionObj{

}




fn index_of(vec: &Vec<Token>,token: Token)->Option<i32>{
    let mut i=0;
    for thing in vec{
        if equals(thing,&token){
            return Some(i);
        }
        i+=1;
    }
    None
}
fn equals(t1:&Token,t2:&Token)->bool {
    std::mem::discriminant(t1) == std::mem::discriminant(t2)
}