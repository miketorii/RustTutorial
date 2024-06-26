use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::mpsc;
//use std::time::Duration;

static NTHREADS: i32 = 3;

fn main() {
    println!("---start---");

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            
            println!("thread {} finished", id);
        });
        
        children.push(child);
    }
    
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("error");
    }
    
    println!("{:?}", ids);
    
    println!("---end---");
}

