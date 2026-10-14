use crate::pyrust::compile_run;

mod pyrust;

fn main() {
    let program= "
scope main(){
    bind num : Int = 0;
    bind text : Str = 'Hello World!';
    label loop;
    text;
    print(it);
    num;
    it+1;
    num=it;
    if(num!=5);
    jump loop;
    else;
    class print(){
        io=
    }
    
}
";
    compile_run(String::from(program))
}
