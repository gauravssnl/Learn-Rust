use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem in parsing arguments: {}", err);
        process::exit(1);
        //  return Config{query: "".to_string(), filename: String::from("")}; // need to uncomment this line if previous line code is not there
    });
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    run(config);
}

struct Config {
    query: String,
    filename: String,
}

// make code more idiomatic
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        //  &str also works
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.filename)?;
    println!("With text: \n{}", contents);
    Ok(())
}
