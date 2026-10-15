use crate::pyrust::compile_run;

mod pyrust;

fn main() {
    let program= "
bind num = 0;
num = 1;
bind cls = {};
";
    compile_run(String::from(program),true)
}
