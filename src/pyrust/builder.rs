use crate::pyrust::parser::*;
use crate::pyrust::parser::Token::*;
pub fn build(mut tokens:Vec<Token>)-> ClassPreProcess {
    let mut class_stack:Vec<ClassPreProcess>=vec![ClassPreProcess::new()];
    loop{
        if tokens.is_empty() { break }
        let mut cur_statement:Vec<Token>=Vec::new();
        loop{
            if tokens.is_empty() { break }
            let cur_token=tokens.remove(0);
            let loc=class_stack.len()-1;
            let mut top:&mut ClassPreProcess=class_stack.get_mut(loc).expect("[ERROR] empty class stack");
            
            match cur_token {
                InsEnd=>{ break }
                ScopeInc=>{
                    push_statement(cur_statement,top);

                    class_stack.push(ClassPreProcess::new());
                    break
                }
                ScopeDec=>{
                    push_statement(cur_statement,&mut top);
                    
                    if class_stack.len()<2 {
                        panic!("[ERROR] too many close brackets")
                    }
                    let to_add =class_stack.remove(class_stack.len()-1);
                    let mut top =class_stack.remove(class_stack.len()-1);
                    top.sub_classes.push(to_add);
                    let num=top.sub_classes.len()-1;
                    class_stack.push(top);
                    
                    break
                }
                _=>{cur_statement.push(cur_token)}
            }
        }
        
    };
    if class_stack.len()>1 {
        panic!("[ERROR] too many open brackets")
    }
    class_stack.remove(0)
}
fn push_statement(statement:Vec<Token>, class: &mut ClassPreProcess){
    class.instructions.push(statement);
}

pub struct ClassPreProcess {
    pub instructions:Vec<Vec<Token>>,
    pub sub_classes:Vec<ClassPreProcess>,
}
impl ClassPreProcess {
    fn new()->Self{
        ClassPreProcess {
            instructions:Vec::new(),
            sub_classes:Vec::new(),
        }
    }
}