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

}
