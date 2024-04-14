use std::mem;

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point {x: 0.0, y: 0.0})
}

fn stdbox() {
    println!("-----Box-----");
    
    // allocated on stack
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle{
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };
    
    //allocated on heap
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle{
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    });
    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    //display
    println!("Point occupies {} bytes on the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack", mem::size_of_val(&rectangle));
    
    //
    println!("Boxed point occupies {} bytes on the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack", mem::size_of_val(&box_in_a_box));

    //
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_point));
    let unboxed_rectangle: Rectangle = *boxed_rectangle;
    println!("Unboxed point occupies {} bytes on the stack", mem::size_of_val(&unboxed_rectangle));
}

////////////////////////////////////////////////////
//
//        
fn stdvector(){
    println!("---vector---");

    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("{:?}", collected_iterator);
    
    let mut xs = vec![1i32, 2, 3];
    println!("initial vector = {:?}", xs);
    
    xs.push(4);
    println!("modified vector = {:?}, Length={}", xs, xs.len());
    println!("xs[1]={}", xs[1]);   
    println!("poped value={:?}", xs.pop());

    for x in xs.iter() {
        println!("> {}", x);
    }
    
    for (i, x) in xs.iter().enumerate(){
        println!("{} : {}", i, x);
    }
    
    for x in xs.iter_mut(){
        *x *= 3;
    }
    println!("modified vector = {:?}, Length={}", xs, xs.len());

}

fn stdstring()
{
    println!("-----String-----");

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    for word in pangram.split_whitespace().rev(){
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    println!("chars: {:?}", chars);
    chars.sort();
    println!("chars: {:?}", chars);
    chars.dedup();
    println!("chars: {:?}", chars);

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("string: {}",string);

    let chars_to_trim: &[char] = &[' ',','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used char: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog","cat");

    println!("alice: {}", alice);
    println!("bob: {}", bob);
}

fn main() {
    println!("-----Start-----");
    stdbox();
    stdvector();
    stdstring();
}