pub struct Term;
impl Term {
    pub fn error(msg: &str) {
        println!(" \x1b[91m\x1b[0m {}", msg);
    }

    pub fn done(msg: &str) {
        println!(" \x1b[92m\x1b[0m {}", msg);
    }

    pub fn list_title(msg: &str) {
        println!(" \x1b[97m\x1b[0m {}", msg);
    }

    pub fn item(msg: &str) {
        println!("   {}", msg)
    }
}
