use std::process::exit;
use crate::pyrust::compiler::*;

pub fn run(main: Class) {
    let root=Scope::new(main);
    let mut scope_stack =vec![root];
    loop{
        step(&mut scope_stack);
    }
    
    
}
fn step(stack:&mut Vec<Scope>){
    let loc=stack.len();
    if loc==0{exit(0)}
    let top_expect =stack.get_mut(loc-1);
    let top=match top_expect {
        Some(e) => {e}
        None => { exit(0) }
    };
    let cur_ins = top.clazz.instructions.get(top.ins_ptr);
    match cur_ins {
        Some(e) => {
            exec(e)
        }
        None =>{
            stack.pop();
            return
        }
    }
    top.ins_ptr+=1;
}
fn exec(instruction:&Instruction){
    println!("{:?}",instruction)
}
struct Scope{
    clazz: Class,
    binds: Vec<(String,Variable)>,
    class_def: Vec<Class>,
    ins_ptr:usize,
}
impl Scope{
    fn new(newtyp: Class) ->Self{
        Scope{
            clazz:newtyp,
            binds:vec![],
            class_def:vec![],
            ins_ptr:0,
        }
    }
}
enum Variable{
    Clazz(i32),
    Scop(Scope),
    Str(String),
    
}