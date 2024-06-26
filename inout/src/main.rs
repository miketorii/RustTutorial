use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

fn main() {
    println!("---start---");

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
   
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumnts: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);

    println!("In file {}", config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }

    println!("---end---");
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("file read error");

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok( Config { query, filename } )
    }
}
