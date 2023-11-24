use std::process::exit;

pub struct Term;
impl Term {
    pub fn error(msg: &str) {
        println!(" \x1b[91mError\x1b[0m: {}", msg);
    }

    pub fn done(msg: &str) {
        println!(" \x1b[92mDone\x1b[0m: {}", msg);
    }

    pub fn busy(msg: &str) {
        println!(" \x1b[97mWork\x1b[0m: {}", msg);
    }

    pub fn list_title(msg: &str) {
        println!(" \x1b[97m\x1b[0m {}", msg);
    }

    pub fn item(msg: &str) {
        println!(" {}", msg)
    }

    pub fn fail(msg: &str) {
        println!(" \x1b[91mFail\x1b[0m: {}", msg);
        exit(1);
    }
}
