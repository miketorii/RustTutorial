use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("---start---");

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
   
    let config = parse_config(&args);

    let query = config.query;
    let filename = config.filename;

    println!("Searching for {}", query);

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("file read error");

    println!("With text:\n{}", contents);

    println!("---end---");
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    
    Config { query, filename }
}
