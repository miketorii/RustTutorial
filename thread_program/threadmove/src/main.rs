use std::thread;

fn main() {
    let v = vec![1,2,3];

    println!("---start---");
    
    let handle = thread::spawn(move || {
        println!("In thread: {:?}", v);
    } );
    
    handle.join().unwrap();
    
    println!("---end---");
}

