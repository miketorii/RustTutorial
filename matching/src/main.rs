enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("-----enum-----");

    let _kind = IpAddrKind::V4;
    let _kind2 = IpAddrKind::V6;

    match _kind2 {
        IpAddrKind::V4 => {println!("v4");},
        IpAddrKind::V6 => {println!("v6");},
    }

    println!("-----Option Some/None-----");

    let some_number = Some(5);
    println!("Some num={:?}", some_number);
    let some_string = Some("string in Some");
    println!("Some num={:?}", some_string);
    let absent_number : Option<i32> = None;
    println!("None num={:?}", absent_number);
}
