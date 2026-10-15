use crate::pyrust::compiler::*;

pub fn run(main: Class) {
    let root=Scope::new(main);
    let mut scope_stack =vec![root];
    loop{
        scope_stack=step(scope_stack);
    }
    
    
}
fn step(mut stack:Vec<Scope>)->Vec<Scope>{
    let mut top =stack.remove(stack.len()-1);
    let cur_ins = top.clazz.instructions.get(top.ins_ptr);
    match cur_ins {
        Some(e) => {
            exec(e)
        }
        None => {
            return stack
        }
    }
    top.ins_ptr+=1;
    stack.push(top);
    stack
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