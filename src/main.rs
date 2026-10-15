use crate::pyrust::compile_run;

mod pyrust;

fn main() {
    let program= "
// bind num = 0;
bind test;
test=1;
// if true;
// bind = 1;
// else;
// bind = 2;
// catch;
// bind text = true;
";
    compile_run(String::from(program),true)
}
