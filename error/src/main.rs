fn drink(beverage: &str){
    if beverage == "lemonade" { panic!("error panic"); };

    println!("not error = {}", beverage);
}

fn main() {
    println!("---panic test---");

    drink("water");
    drink("lemonade");
}
