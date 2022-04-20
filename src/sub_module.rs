pub fn print(msg: &str) {
    println!("{}", msg);
}

pub mod submodule {
    pub const MSG: &str = "Hello world!";
}
