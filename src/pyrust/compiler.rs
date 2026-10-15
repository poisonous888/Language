use crate::pyrust::builder::*;
use crate::pyrust::compiler::Instruction::*;
use crate::pyrust::compiler::Literal::*;
use crate::pyrust::parser::*;
use crate::pyrust::parser::Opperator::*;
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
    let first=match statement.first() {
        Some(e) => {e}
        None => {return instruction_set}
    };
    //binds
    let binds=index_mult(&statement,Bind);
    if binds.len()>1 {panic!("[COMPILE ERROR] cannot bind multiple times in the same statement")}
    if binds.len()==1{
        let next=statement.get(1);
        match next {
            Some(Identifier(e))=>{
                if debug{println!(" - name: {}",e)}
                instruction_set.push(BindNew(e.clone()));
            }
            _=>{panic!("[COMPILE ERROR] cannot bind to non-identifier")}
        }
    }
    //assigns
    let assigns=index_mult(&statement,Assign);
    if assigns.len()>1 {panic!("[COMPILE ERROR] cannot assign multiple times in the same statement")}
    for thing in assigns{
        let prev=statement.get(thing as usize-1);
        match prev {
            Some(Identifier(e))=>{
                let sub=get_after(&statement,thing as usize);
                let mut expr=get_inst(sub,debug);
                instruction_set.append(&mut expr);
                instruction_set.push(BindAssign(e.clone()))
            }
            _=>{panic!("[COMPILE ERROR] cannot assign to non-identifier")}
        }
    }
    //expressions
    let mut opperands =index_mult(&statement,Oppr(Add));
    loop{
        if opperands.len()==0{break}
        let cur=opperands.remove(0);
        let prev=statement.get(cur as usize-1);
        let prev_uw=match prev {
            Some(e)=>{e}
            None=>{panic!("[COMPILE ERROR] opperand must have a left side")}
        };
        let prev_ins=match prev_uw {
            Identifier(_)|StringLit(_)|NumLit(_)|BoolLit(_)=>{
                get_inst(vec![prev_uw.clone()],debug)
            }
            _=>{panic!("[COMPILE ERROR] invalid opperand left side")}
        };
    }
    match index_of(&statement,Oppr(Add)){
        None=>{}
        Some(i)=>{
            if debug{println!("Expression")}
            if i!=1{panic!("[COMPILE ERROR] expression must be between 2 other tokens")}
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
                _=>{panic!("[COMPILE ERROR] cannot express on that token")}
            }
        }
    }
    match first {
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
    instruction_set
}
#[derive(Debug)]
pub enum Instruction{
    BindNew(String),
    BindAssign(String),
    PushBind(String),
    PushLiteral(Literal),
    Math(Opperator),
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
fn get_after(vec: &Vec<Token>,start:usize)->Vec<Token>{
    let mut sub =Vec::new();
    let mut i = start+1;
    loop{
        match vec.get(i){
            None=>{break}
            Some(w)=>{
                sub.push(w.clone());
                i+=1;
            }
        }
    }
    sub
}
fn index_mult(vec: &Vec<Token>,token: Token)->Vec<i32>{
    let mut out:Vec<i32>=Vec::new();
    let mut i=0;
    for thing in vec {
        if equals(thing,&token){
            out.push(i);
        }
        i+=1;
    }
    out
}
fn equals(t1:&Token,t2:&Token)->bool {
    std::mem::discriminant(t1) == std::mem::discriminant(t2)
}