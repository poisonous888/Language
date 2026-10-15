use crate::pyrust::compile_run;

mod pyrust;

fn main() {
    let program= "
bind num = 0;
bind text='Hello World!';
//test
num = 1;
io=text;
bind newScope = {
    return;
}
";
    compile_run(String::from(program))
}
