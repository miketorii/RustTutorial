use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    println!("---start---");

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
   
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumnts: {}", err);
        process::exit(1);
    });

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
