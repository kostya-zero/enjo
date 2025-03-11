use std::process::exit;

use anyhow::Result;
use enjo::terminal::Message;

fn main() {
    let result: Result<(), anyhow::Error> = enjo::app::run();
    if let Err(e) = result {
        Message::error(e.to_string().as_str());
        exit(1);
    }
}
