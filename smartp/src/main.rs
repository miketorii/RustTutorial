use std::ops::Deref;

use std::rc::Rc;

use std::time::Duration;
use std::sync::Arc;
use std::thread;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello, {}", name);
}

fn dereftrait()
{
    println!("---deref trait---");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{} {}", x, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

//////////////////////////////////////////////
//
struct MyPtr {
    data: String,
}

impl Drop for MyPtr {
    fn drop(&mut self){
        println!("dropping MyPtr in drop() : {}", self.data);
    }
}
fn droptrait()
{
    println!("---drop trait---");

    let c = MyPtr { data: String::from("my stuff")};
    let d = MyPtr { data: String::from("other stuff")};
    println!("created MyPtr");

    println!("---created some stuff---");
    let e = MyPtr { data: String::from("some stuff") };
    println!("before calling drop()");
    drop(e);
    println!("drop() was called");

    println!("---End: drop trait---");
}

//////////////////////////////////////////////
//
fn rcbehave()
{
    println!("---Rc behave---");

    let rc_examples = "Rc examples".to_string();

    let rc_a: Rc<String> = Rc::new(rc_examples);
    println!("ref count rc_a: {}", Rc::strong_count(&rc_a) );

    {
        let rc_b: Rc<String> = Rc::clone(&rc_a);
        println!("ref count rc_b: {}", Rc::strong_count(&rc_b) );
        println!("ref count rc_a: {}", Rc::strong_count(&rc_a) );

        println!("rc_a = rc_b : {}", rc_a.eq(&rc_b));

        println!("rc_a length = {}", rc_a.len());
        println!("rc_b val = {}", rc_b);
    }

    println!("ref count rc_a: {}", Rc::strong_count(&rc_a) );
}

//////////////////////////////////////////////
//
fn arcbehave()
{
    println!("---Arc behave---");

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple1 = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?} : ref count={}", apple1, Arc::strong_count(&apple1) );
        } );
    }

    thread::sleep(Duration::from_secs(1));
}

//////////////////////////////////////////////
//
fn main() {
    println!("---Start---");

    dereftrait();
    droptrait();
    rcbehave();
    arcbehave();

    println!("---End---");
}
