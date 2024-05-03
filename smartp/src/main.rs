use std::ops::Deref;

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

fn main() {
    println!("---Start---");

    dereftrait();
    droptrait();

    println!("---End---");
}