use crate::pyrust::parser::SpecialIdentifier::*;
use crate::pyrust::parser::Token::*;
pub fn parse(string: String){
    let mut test =tokenize(string);
    loop{
        if test.len()==0 { break; }
        let out=match test.remove(0) {
            Identifier(e) => { "identifier - ".to_owned()+&*e }
            Special(e) => {
                "special identifier - ".to_owned()+match e {
                    It => {"it"}
                    Io => {"io"}
                }
            }
            StringLit(e) => {"string literal - ".to_owned()+&*e}
            InsEnd => {";".to_owned()}
            Bind => {"bind".to_owned()}
            Class => {"class".to_owned()}
            Scope => {"scope".to_owned()}
            ScopeInc => {"{".to_owned()}
            ScopeDec => {"}".to_owned()}
            Label => {"label".to_owned()}
            Jump => {"jump".to_owned()}
            If => {"if".to_owned()}
            Else => {"else".to_owned()}
            Catch => {"catch".to_owned()}
        };
        println!("{}", out);
    }
}
fn tokenize(string: String)->Vec<Token>{
    let mut chars =string.chars();
    let mut output:Vec<Token>=vec![];
    let mut cur_token=String::new();
    let mut string_lit=false;
    let mut comment=false;
    loop{
        let next=chars.next();
        let cur:char;
        match next {
            Some(it)=> { cur = it }
            None=>{break}
        }
        if comment{
            match cur {
                '/'|'\n'=> { comment = false }
                _=>{}
            }
            continue
        }
        if string_lit{
            match cur {
                '\''|'\"'=>{
                    output.push(StringLit(cur_token));
                    cur_token=String::new();
                    string_lit=false;
                }
                i=>{cur_token.push(i)}
            }
            continue
        }
        if cur.is_whitespace() {
            cur_token=push_token(cur_token,&mut output);
            continue
        }
        match cur {
            '/'=>{comment=true}
            '\''|'\"'=>{string_lit=true}
            '{'=>{
                cur_token=push_token(cur_token,&mut output);
                output.push(ScopeInc);
            }
            '}'=>{
                cur_token=push_token(cur_token,&mut output);
                output.push(ScopeDec);
            }
            ';'=>{
                cur_token=push_token(cur_token,&mut output);
                output.push(InsEnd);
            }
            i=>{cur_token.push(i) }
        }
    }
    output
}
fn push_token(token:String, stack: &mut Vec<Token>) ->String{
    if token.is_empty() {}
    else if token=="bind"{stack.push(Bind)}
    else if token=="label"{stack.push(Label)}
    else if token=="jump"{stack.push(Jump)}
    else if token=="if"{stack.push(If)}
    else if token=="else"{stack.push(Else)}
    else if token=="catch"{stack.push(Catch)}
    else if token=="it"{stack.push(Special(It))}
    else if token=="io"{stack.push(Special(Io))}
    else{stack.push(Identifier(token))}
    String::new()
}
enum Token{
    Identifier(String), //anything that can have a name
    Special(SpecialIdentifier), //see below
    StringLit(String), //""
    InsEnd, //;
    
    Bind,Class,Scope, //bind is variables, class is a java class, scope is a kotlin object
    ScopeInc,ScopeDec, //scope and class things
    
    Label,Jump, //standard assembly shenanigans
    If,Else,Catch, //control flow but without the brackets
}
enum SpecialIdentifier{
    It, //the weird underlying stack, writing to it pushes to the stack, reading from it pops from the stack
    Io, //input and output, writing to it prints to terminal, reading from it gets the terminal input
}