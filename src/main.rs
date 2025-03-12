use std::env;
use std::process;
use tzcon::convert;
use tzcon::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error: {err}");
        process::exit(1);
    });

    convert(config).format_output();
}

// TODO:
// 1. 24 hour, 12 hour
