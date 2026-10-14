fn run() {
    
    
    
    
    
    
}
struct Scope{
    binds: Vec<(String,String)>,
    subscopes: Vec<Scope>,
    ins_ptr:i32,
}
impl Scope{
    fn new()->Self{
        Scope{
            binds:vec![],
            subscopes:vec![],
            ins_ptr:0,
        }
    }
}