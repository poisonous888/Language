use crate::pyrust::parser::MatchType::*;
use crate::pyrust::parser::Number::*;
use crate::pyrust::parser::Opperator::*;
use crate::pyrust::parser::Token::*;
pub fn tokenize(string: String,debug:bool)->Vec<Token>{
    let mut chars =string.chars();
    let mut output:Vec<Token>=vec![];
    let mut cur_token=String::new();
    let mut state=Normal;
    let mut next_override=false;
    let mut next:Option<char>=None;
    loop{
        if next_override {next_override=false}
        else {next = chars.next()}
        let cur:char;
        match next {
            Some(it)=> { cur = it }
            None=>{break}
        }
        match state {
            Normal => {
                if cur.is_whitespace() {
                    cur_token=push_token(cur_token,&mut output);
                    continue
                }
                match cur {
                    '\''|'\"'=>{state=StringLiteral}
                    '='|'*'|'/'|'+'|'-'|'!'|'^'|'%'|'>'|'<'=>{
                        cur_token=push_token(cur_token,&mut output);
                        cur_token.push(cur);
                        state=Opp
                    }
                    '0'..='9'=>{
                        if cur_token.is_empty() {
                            state=Number;
                        }
                        cur_token.push(cur)
                    }
                    '.'=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(IntoScope);
                    }
                    '{'=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(ScopeInc);
                    }
                    '}'=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(ScopeDec);
                    }
                    '('=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(ArgStart);
                    }
                    ')'=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(ArgEnd);
                    }
                    ';'=>{
                        cur_token=push_token(cur_token,&mut output);
                        output.push(InsEnd);
                    }
                    _=>{cur_token.push(cur) }
                }
            }
            StringLiteral => {
                match cur {
                    '\''|'\"'=>{
                        output.push(StringLit(cur_token));
                        cur_token=String::new();
                        state=Normal
                    }
                    _=>{cur_token.push(cur)}
                }
            }
            Number => {
                match cur {
                    '0'..='9'|'.'|','=>{
                        cur_token.push(cur)
                    }
                    _=>{
                        output.push(NumLit(parse_number(cur_token)));
                        cur_token=String::new();
                        next_override=true;
                        state=Normal;
                    }
                }
            }
            Opp => {
                match cur {
                    '='|'+'|'-'|'/'=>{
                        cur_token.push(cur);
                    }
                    _=>{
                        next_override=true
                    }
                }
                let new_type =match cur_token.as_str() {
                    "//"=>{
                        state=Comment;
                        cur_token=String::new();
                        continue
                    }
                    "="=>{
                        output.push(Assign);
                        cur_token=String::new();
                        state=Normal;
                        continue
                    }
                    "=="=>{Equal}
                    ">"=>{Greater}
                    "<"=>{Less}
                    ">="=>{GreaterEq}
                    "<="=>{LessEq}
                    "!"=>{Not}
                    "!="=>{NotEqual}
                    "++"=>{Inc}
                    "--"=>{Dec}
                    "+"=>{Add}
                    "-"=>{Sub}
                    "*"=>{Mult}
                    "/"=>{Div}
                    "+="=>{SetAdd}
                    "-="=>{SetSub}
                    "*="=>{SetMult}
                    "/="=>{SetDiv}
                    "%"=>{Mod}
                    _=>{panic!("Bad Operator")}
                };
                output.push(Oppr(new_type));
                cur_token=String::new();
                state=Normal;
            }
            Comment => {
                match cur {
                    '\n'=> { state=Normal }
                    _=>{}
                }
            }
        }
        if debug{println!("{:?} - {}", state, cur_token)}
    }
    if debug{println!("Parsed: {:?}", output)}
    output
}
#[derive(Debug)]
enum MatchType{
    Normal,
    Number,
    StringLiteral,
    Opp,
    Comment,
}
fn push_token(token:String, stack: &mut Vec<Token>) ->String{
    if token.is_empty() {}
    else if token=="true"{stack.push(BoolLit(true))}
    else if token=="false"{stack.push(BoolLit(false))}
    else if token=="bind"{stack.push(Bind)}
    else if token=="label"{stack.push(Label)}
    else if token=="return"{stack.push(Return)}
    else if token=="jump"{stack.push(Jump)}
    else if token=="if"{stack.push(If)}
    else if token=="else"{stack.push(Else)}
    else if token=="catch"{stack.push(Catch)}
    else{stack.push(Identifier(token))}
    String::new()
}
#[derive(Debug, Clone)]
pub enum Token{
    Identifier(String), //anything that can have a name
    Assign,
    StringLit(String), //""
    NumLit(Number),
    BoolLit(bool),
    Oppr(Opperator),
    InsEnd, //;
    IntoScope, //.
    
    Bind, //bind is variables
    Scope(i32),ScopeInc,ScopeDec,
    Return, //scope and class things
    Arg(Vec<String>),ArgStart,ArgEnd, //parentheses
    
    Label,Jump, //standard assembly shenanigans
    If,Else,Catch, //control flow but without the brackets
}
#[derive(Debug, Clone)]
pub enum Number{
    Int(i32),
    Double(f64)
}
#[derive(Debug, Clone)]
pub enum Opperator{
    Not,
    Equal,
    NotEqual,
    Greater,Less,
    GreaterEq,LessEq,
    Inc,Dec,
    Add,Sub,Mult,Div,
    SetAdd,SetSub,SetMult,SetDiv,
    Mod,
}
fn parse_number(str:String)->Number{
    if str.contains('.') {
        let out:f64=str.parse().expect(&*("Invalid Double - ".to_owned()+&*str));
        return Double(out);
    }
    let out:i32=str.parse().expect(&*("Invalid Integer - ".to_owned()+&*str));
    Int(out)
}