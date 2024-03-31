fn main() {
    println!("---main binary---");

    barlib::barmethod();

    let result = barlib::add(3,2);
    println!("result={}", result);
}
