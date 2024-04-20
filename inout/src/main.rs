use std::env;

fn main() {
    println!("---start---");

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    println!("---end---");
}
