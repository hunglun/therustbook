use std::env;
use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    run(config);
}

    // parse argument
    // call worker thread to process the args
    // handle exception

