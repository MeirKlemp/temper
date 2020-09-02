use std::{env, process};

use temper::Config;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();

    let config = Config::new(args).unwrap_or_else(|err| {
        Config::usage(&prog_name);
        eprintln!("Arguments error: {}", err);
        process::exit(1);
    });

    if let Err(e) = temper::run(config) {
        Config::usage(&prog_name);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
