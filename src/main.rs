use std::process::exit;

extern crate enjo;

fn main() {
    let result = enjo::main();

    if let Err(err) = result {
        enjo::term::Message::fail(err.to_string().as_str());
    }

    exit(0);
}
