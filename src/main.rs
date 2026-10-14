use crate::pyrust::compile_run;

mod pyrust;

fn main() {
    let program= "
bind num = 0;
bind text = 'Hello World!';
io=text;
";
    compile_run(String::from(program))
}
