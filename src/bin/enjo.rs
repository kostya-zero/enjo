use std::process::exit;

use enjo::{
    app::App, args::get_args, config::Config, platform::Platform, term::Message, utils::Utils,
};

extern crate enjo;

fn main() {
    let args = get_args().get_matches();
    if !Platform::check_exists() {
        let default_config: Config = Config::default();
        Utils::write_config(default_config);

        Message::info(
            "Enjo has generated the default configuration. Change it according to your needs.",
        );
    }

    let config: Config = Utils::get_config();

    let result = App::new(config, args).run();
    if let Err(err) = result {
        Message::fail(err.to_string().as_str());
    }

    exit(0);
}
