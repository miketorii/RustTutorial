//////////////////////////////////////////////////////////////////
//
//
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//////////////////////////////////////////////////////////////////
//
//
struct Sheep {
    naked: bool,
    name: &'static str
}

trait Animal {
    fn new(name: &'static str) -> Self;
    
    fn name(&self) -> &'static str;
    
    fn talk(&self){
        println!("Animal::talk");
    }
}

impl Sheep {
    fn shear(&mut self){
        println!("{}", self.name());
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }
    
    fn name(&self) -> &'static str {
        self.name
    }
}

//////////////////////////////////////////////////////////////////
//
//
fn main() {
    println!("---trait---");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("this is a horse e book."),
        reply: false,
        retweet: false,
    };

    println!("I new tweet: {}", tweet.summarize());

    println!("-----trait example-----");
    
    let mut dolly: Sheep = Animal::new("Dolly");
 
    dolly.talk();
    dolly.shear();

    println!("{} {}", dolly.name, dolly.naked);
}
