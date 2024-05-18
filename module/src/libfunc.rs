pub fn libfuncA()
{
    println!("--libfuncA--")
}

pub fn libfuncAdd(left: usize, right: usize) -> usize {
    left + right
}

pub fn libfuncReturn10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
