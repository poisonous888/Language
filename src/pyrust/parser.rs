use crate::pyrust::parser::MatchType::*;
use crate::pyrust::parser::Number::*;
use crate::pyrust::parser::Opperator::*;
use crate::pyrust::parser::Token::*;
pub fn parse(string: String){
    let mut test=tokenize(string);
    println!();
    println!("PRINTING");
    println!();
    loop{
        if test.len()==0 { break; }
        print(&test.remove(0));
    }
}
fn tokenize(string: String)->Vec<Token>{
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
                    '='|'+'|'-'=>{
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
                    "="=>{Assign}
                    "=="=>{Equal}
                    ">"=>{Greater}
                    "<"=>{Less}
                    ">="=>{GreaterEq}
                    "<="=>{LessEq}
                    "!"=>{Not}
                    "!="=>{NotEqual}
                    "++"=>{Inc}
                    "--"=>{Sub}
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
                    '/'|'\n'=> { state=Normal }
                    _=>{}
                }
            }
        }
        println!("{:?} - {}",state, cur_token)
    }
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
    else if token=="bind"{stack.push(Bind)}
    else if token=="label"{stack.push(Label)}
    else if token=="jump"{stack.push(Jump)}
    else if token=="if"{stack.push(If)}
    else if token=="else"{stack.push(Else)}
    else if token=="catch"{stack.push(Catch)}
    else{stack.push(Identifier(token))}
    String::new()
}
enum Token{
    Identifier(String), //anything that can have a name
    StringLit(String), //""
    NumLit(Number),
    Oppr(Opperator),
    InsEnd, //;
    IntoScope, //.
    
    Bind,Class,Scope, //bind is variables, class is a java class, scope is a kotlin object
    ScopeInc,ScopeDec, //scope and class things
    ArgStart,ArgEnd, //parentheses
    
    Label,Jump, //standard assembly shenanigans
    If,Else,Catch, //control flow but without the brackets
}
enum Number{
    Int(i32),
    Double(f64)
}
enum Opperator{
    Assign,
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

fn print(cur:&Token){
    let out=match cur {
        Identifier(e) => { "identifier - ".to_owned()+&*e }
        StringLit(e) => {"string literal - ".to_string()+ &*e }
        InsEnd => {";".to_owned()}
        Bind => {"bind".to_owned()}
        Class => {"class".to_owned()}
        Scope => {"scope".to_owned()}
        IntoScope => {"into scope".to_owned()}
        ScopeInc => {"{".to_owned()}
        ScopeDec => {"}".to_owned()}
        ArgStart => {"(".to_owned()}
        ArgEnd => {")".to_owned()}
        Label => {"label".to_owned()}
        Jump => {"jump".to_owned()}
        If => {"if".to_owned()}
        Else => {"else".to_owned()}
        Catch => {"catch".to_owned()}
        NumLit(e) => {match e {
            Int(e) => { "int literal - ".to_owned()+ &*e.to_string() }
            Double(e) => { "double literal - ".to_owned()+ &*e.to_string() }
        }}
        Oppr(e)=>{
            match e {
                Assign => {"assign".to_string()}
                Not => {"!".to_string()}
                Equal => {"==".to_string()}
                NotEqual => {"!=".to_string()}
                Greater => {">".to_string()}
                Less => {"<".to_string()}
                GreaterEq => {">=".to_string()}
                LessEq => {"<=>".to_string()}
                Inc => {"++".to_string()}
                Dec => {"--".to_string()}
                Add => {"+".to_string()}
                Sub => {"-".to_string()}
                Mult => {"*".to_string()}
                Div => {"/".to_string()}
                SetAdd => {"+=".to_string()}
                SetSub => {"-=".to_string()}
                SetMult => {"*=".to_string()}
                SetDiv => {"/=".to_string()}
                Mod => {"%".to_string()}
            }
        }
    };
    println!("{}", out);
}
