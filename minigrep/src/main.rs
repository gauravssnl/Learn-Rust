use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem in parsing arguments: {}", err);
        process::exit(1);
        //  return Config{query: "".to_string(), filename: String::from("")}; // need to uncomment this line if previous line code is not there
    });
    // println!("Searching for: {}", config.query);
    // println!("In file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
